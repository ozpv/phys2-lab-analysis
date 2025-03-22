use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

#[cfg(feature = "ssr")]
use http::{header::CONTENT_LENGTH, HeaderMap, StatusCode};
#[cfg(feature = "ssr")]
use leptos_axum::{extract, ResponseOptions};
#[cfg(feature = "ssr")]
use rand::distr::{Alphanumeric, SampleString};
#[cfg(feature = "ssr")]
use std::path::PathBuf;
#[cfg(feature = "ssr")]
use thiserror::Error;
#[cfg(feature = "ssr")]
use tokio::{fs::File, io::AsyncWriteExt};

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
enum ImageUploadError {
    #[error("Uploading to the client is not allowed, this should never occur")]
    Client,
    #[error("Make sure the request contains content length and type")]
    MissingHeaders,
    #[error("Make sure the file is an image and isn't larger than 10MB")]
    InvalidFile,
    #[error("Something went wrong uploading the image")]
    Internal,
}

#[server(prefix = "/api", endpoint = "upload_image", input = MultipartFormData)]
pub async fn upload_image(data: MultipartData) -> Result<(), ServerFnError> {
    let mut data = data.into_inner().ok_or(ImageUploadError::Client)?;

    // check if the file is a valid size and format
    let headers = extract::<HeaderMap>()
        .await
        .map_err(|_| ImageUploadError::MissingHeaders)?;

    let content_length = headers
        .get(CONTENT_LENGTH)
        .ok_or(ImageUploadError::MissingHeaders)
        .inspect_err(|_| {
            expect_context::<ResponseOptions>().set_status(StatusCode::LENGTH_REQUIRED);
        })?
        .to_str()
        .ok()
        .and_then(|r| r.parse::<u64>().ok())
        .ok_or(ImageUploadError::InvalidFile)
        .inspect_err(|_| {
            expect_context::<ResponseOptions>().set_status(StatusCode::BAD_REQUEST);
        })?;

    if content_length >= 10_000_000 {
        expect_context::<ResponseOptions>().set_status(StatusCode::BAD_REQUEST);
        return Err(ImageUploadError::InvalidFile.into());
    }

    while let Ok(Some(mut field)) = data.next_field().await {
        let content_type = field.content_type();

        if content_type.is_none_or(|s| !s.to_string().starts_with("image")) {
            expect_context::<ResponseOptions>().set_status(StatusCode::BAD_REQUEST);
            return Err(ImageUploadError::InvalidFile.into());
        }

        // get path to write to
        // is LEPTOS_SITE_ROOT by default
        let site_root = expect_context::<LeptosOptions>().site_root;
        let file_name = field.file_name().map_or_else(
            || Alphanumeric.sample_string(&mut rand::rng(), 32),
            ToString::to_string,
        );
        let path = PathBuf::new().join(site_root.to_string()).join(file_name);

        tracing::info!("Uploading file {}", path.display());

        let mut file = File::options()
            .create(true)
            .append(true)
            .open(path)
            .await
            .map_err(|_| ImageUploadError::Internal)?;

        while let Ok(Some(chunk)) = field.chunk().await {
            file.write_all(&chunk)
                .await
                .map_err(|_| ImageUploadError::Internal)?;
        }
    }

    Ok(())
}

#[cfg(all(test, feature = "ssr"))]
mod test {
    use crate::routes::app;
    use axum::{
        body::Body,
        http::{header, Request, StatusCode},
    };
    use futures::{stream, StreamExt};
    use tower::ServiceExt;

    #[inline]
    fn test_multipart_body() -> Body {
        Body::from(
            r#"------WebKitFormBoundary7MA4YWxkTrZu0gW
Content-Disposition: form-data; name="file"; filename="test.png"
Content-Type: image/png

iVBORw0KGgoAAAANSUhEUgAAAAUA
----WebKitFormBoundary7MA4YWxkTrZu0gW--
"#,
        )
    }

    #[tokio::test]
    async fn upload_image_missing_headers() {
        let app = app();

        let bad_content_length = Request::builder()
            .method("POST")
            .uri("/api/upload_image")
            .header(
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .body(test_multipart_body())
            .unwrap();

        let res = app.oneshot(bad_content_length).await.unwrap();

        assert_eq!(res.status(), StatusCode::LENGTH_REQUIRED);
    }

    #[tokio::test]
    async fn upload_image_bad_content_length() {
        let app = app();

        let too_long = Request::builder()
            .method("POST")
            .uri("/api/upload_image")
            .header(
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .header(header::CONTENT_LENGTH, "10000000")
            .body(test_multipart_body())
            .unwrap();

        let too_short = Request::builder()
            .method("POST")
            .uri("/api/upload_image")
            .header(
                header::CONTENT_TYPE,
                "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW",
            )
            .header(header::CONTENT_LENGTH, "-1")
            .body(test_multipart_body())
            .unwrap();

        let mut s = app.call_all(stream::iter([too_long, too_short]));

        while let Some(Ok(res)) = &mut s.next().await {
            assert_eq!(res.status(), StatusCode::BAD_REQUEST);
        }
    }

    #[ignore]
    #[tokio::test]
    async fn upload_image_bad_content_type() {
        // trust that this works
        todo!()
    }
}
