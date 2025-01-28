use leptos::{ev::SubmitEvent, html, prelude::*, web_sys::FormData};
use server_fn::codec::{MultipartData, MultipartFormData};

#[cfg(feature = "ssr")]
use http::StatusCode;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
#[allow(unused)]
use std::path::{Path, PathBuf};
#[cfg(feature = "ssr")]
use thiserror::Error;

#[server(input = MultipartFormData)]
pub async fn upload_image(data: MultipartData) -> Result<(), ServerFnError> {
    Ok(())
}

#[cfg(feature = "ssr")]
#[derive(Debug, Error)]
enum EncodeAsWebPError {
    #[error("file_path is missing stem")]
    MissingStem,
    #[error("Invalid file_path")]
    InvalidPath,
}

#[server]
pub async fn encode_as_webp(file_path: PathBuf) -> Result<(), ServerFnError> {
    crate::core::utils::validate_path(&file_path).map_err(|_| EncodeAsWebPError::InvalidPath)?;

    let stem = file_path.file_stem().ok_or_else(|| {
        expect_context::<ResponseOptions>().set_status(StatusCode::BAD_REQUEST);
        EncodeAsWebPError::MissingStem
    })?;

    let parent = file_path.parent();

    let new_path = PathBuf::new()
        .join(parent.unwrap_or_else(|| Path::new("")))
        .join(stem)
        .with_extension("webp");

    tracing::info!(
        "re-writing {} to {}",
        file_path.display(),
        new_path.display()
    );

    Ok(())
}

#[component]
pub fn ImageUploadForm() -> impl IntoView {
    let upload = Action::new_local(|data: &FormData| upload_image(data.clone().into()));

    let test = Action::new_local(|name: &String| encode_as_webp(name.clone().into()));
    let node_ref = NodeRef::<html::Input>::new();
    let on_click_test = move |ev: SubmitEvent| {
        ev.prevent_default();
        let path = node_ref.get().expect("<input> should exist").value();
        test.dispatch(path);
    };

    view! {
        <p>"This is an upload area"</p>
        <form on:submit=on_click_test>
            <input type="text" node_ref=node_ref/>
            <button type="submit" class="text-center text-text-dark p-5 rounded rounded-lg bg-base-dark">"Submit"</button>
        </form>
    }
}
