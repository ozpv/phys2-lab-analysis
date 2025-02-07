use leptos::{
    ev::SubmitEvent,
    prelude::*,
    web_sys::{FormData, HtmlFormElement},
};
use wasm_bindgen::JsCast;

use crate::api::upload::upload_image;
use crate::components::buttons::{Button, ButtonVariant};

#[component]
pub fn ImageUploadForm() -> impl IntoView {
    let upload = Action::new_local(|data: &FormData| upload_image(data.clone().into()));

    let on_click = move |ev: SubmitEvent| {
        ev.prevent_default();
        let element = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&element).unwrap();
        upload.dispatch_local(data);
    };

    view! {
        <form on:submit=on_click>
            <input type="file" name="file-upload" />
            <Button {..} type="submit">"Upload"</Button>
        </form>
    }
}
