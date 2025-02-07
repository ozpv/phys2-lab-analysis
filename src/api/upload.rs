use leptos::prelude::*;
use server_fn::codec::{MultipartData, MultipartFormData};

#[cfg(feature = "ssr")]
use http::{header::CONTENT_LENGTH, HeaderMap, StatusCode};
#[cfg(feature = "ssr")]
use leptos_axum::{extract, ResponseOptions};
#[cfg(feature = "ssr")]
use std::path::{Path, PathBuf};
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

#[server(input = MultipartFormData)]
pub async fn upload_image(data: MultipartData) -> Result<(), ServerFnError> {
    let mut data = data.into_inner().ok_or_else(|| ImageUploadError::Client)?;

    while let Ok(Some(mut field)) = data.next_field().await {
        // check if the file is a valid size and format
        let headers = extract::<HeaderMap>()
            .await
            .map_err(|_| ImageUploadError::MissingHeaders)?;

        // check if file is an image and 10MB or under
        if headers
            .get(CONTENT_LENGTH)
            .ok_or_else(|| {
                expect_context::<ResponseOptions>().set_status(StatusCode::LENGTH_REQUIRED);
                ImageUploadError::MissingHeaders
            })
            .and_then(|length| {
                // error by setting content length above max
                Ok(length
                    .to_str()
                    .ok()
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(10000001))
            })?
            >= 10000000
            || !field
                .content_type()
                .is_some_and(|t| t.to_string().starts_with("image"))
        {
            expect_context::<ResponseOptions>().set_status(StatusCode::BAD_REQUEST);
            return Err(ImageUploadError::InvalidFile.into());
        }

        // get path to write to
        // is LEPTOS_SITE_ROOT by default
        let site_root = expect_context::<LeptosOptions>().site_root;
        let file_name = field.file_name().unwrap_or("some_file").to_string();
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
