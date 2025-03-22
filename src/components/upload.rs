use leptos::{
    ev::SubmitEvent,
    html,
    prelude::*,
    web_sys::{FormData, HtmlFormElement},
};
use leptos_icons::Icon;
use wasm_bindgen::JsCast;

use crate::api::upload::upload_image;
use crate::components::buttons::Button;

#[component]
pub fn ImageUploadForm() -> impl IntoView {
    let upload = Action::new_local(|data: &FormData| upload_image(data.clone().into()));

    let on_click = move |ev: SubmitEvent| {
        ev.prevent_default();
        let element = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&element).unwrap();
        upload.dispatch_local(data);
    };

    let input_element = NodeRef::<html::Input>::new();

    view! {
        <div class="bg-base p-8 rounded-lg shadow-lg w-full max-w-md">
            <form class="border-2 border-dashed border-crust rounded-lg p-6 text-center transition duration-200 hover:border-sky"
                on:submit=on_click
            >
                <Icon icon={icondata::LuUpload} width="36" height="36" {..} class="text-text hover:text-sky"/>
                <p class="mt-2 text-text">"Drag and drop an image here"</p>
                <p class="text-sm text-subtext">"or"</p>
                <Button {..} type="button"
                    on:click=move |ev| {
                        ev.prevent_default();

                        let element = input_element.get().expect("<input> to exist");
                        element.click();
                    }
                >
                    "Browse Files"
                </Button>
                <input type="file" class="hidden" accept="image/*" node_ref=input_element />
            </form>
        </div>
    }
}
