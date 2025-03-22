use leptos::{
    ev::{DragEvent, SubmitEvent},
    html,
    leptos_dom::helpers,
    prelude::*,
    web_sys::{Event, FormData, HtmlElement, HtmlFormElement, HtmlInputElement},
};
use leptos_icons::Icon;
use std::sync::Arc;
use wasm_bindgen::JsCast;

use crate::api::upload::upload_image;
use crate::components::buttons::{Button, ButtonVariant};
use crate::components::dialog::DialogWindow;

#[component]
pub fn ImageUploadForm() -> impl IntoView {
    let upload = Action::new_local(|data: &FormData| upload_image(data.clone().into()));

    let status = Arc::new(RwSignal::new(false));

    let input_element = NodeRef::<html::Input>::new();
    let selected_files = NodeRef::<html::P>::new();
    let form_element = NodeRef::<html::Form>::new();

    // when the upload button is clicked
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let element = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let data = FormData::new_with_form(&element).unwrap();
        upload.dispatch_local(data);
    };

    let on_dragover = move |ev: DragEvent| {
        ev.prevent_default();

        let form = form_element.get().unwrap().unchecked_into::<HtmlElement>();
        form.class_list().add_1("border-sky").unwrap();

        let icon = helpers::document()
            .get_element_by_id("icon_element")
            .unwrap();

        icon.class_list().remove_1("text-text").unwrap();
        icon.class_list().add_1("text-sky").unwrap();
    };

    let on_drop = move |ev: DragEvent| {
        ev.prevent_default();

        let input = input_element.get().unwrap();

        // set the files of the input
        let files = ev.data_transfer().unwrap().files();
        input.set_files(files.as_ref());

        // send a change event
        let change_ev = Event::new("change").unwrap();
        input.dispatch_event(&change_ev).unwrap();

        let form = form_element.get().unwrap();
        form.class_list().remove_1("border-sky").unwrap();

        let icon = helpers::document()
            .get_element_by_id("icon_element")
            .unwrap();
        icon.class_list().remove_1("text-sky").unwrap();
        icon.class_list().add_1("text-text").unwrap();
    };

    let on_dragleave = move |_| {
        let form = form_element.get().unwrap().unchecked_into::<HtmlElement>();
        let icon = helpers::document()
            .get_element_by_id("icon_element")
            .unwrap();

        form.class_list().remove_1("border-sky").unwrap();

        icon.class_list().remove_1("text-sky").unwrap();
        icon.class_list().add_1("text-text").unwrap();
    };

    // for when files are selected
    let binding = Arc::clone(&status);
    let on_change = move |ev: Event| {
        let files = ev
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .files()
            .unwrap();

        if files.length() == 1 {
            // setup dialog window
            let file = files.get(0).unwrap();
            let p = selected_files.get().unwrap();

            Dom::set_inner_html(&p, &file.name());

            // open dialog window
            binding.set(true);
        }
        // TODO add a too many files window
    };

    // click event to close the window
    let binding = Arc::clone(&status);
    let close_window = move |_| {
        binding.set(false);
    };

    view! {
        <div class="bg-base p-8 rounded-lg shadow-lg w-full max-w-md">
            <a class="border-sky text-sky"/>
            <form class="flex flex-col items-center border-2 border-dashed border-crust rounded-lg p-6 text-center transition duration-200"
                on:submit=on_submit
                on:drop=on_drop
                on:dragover=on_dragover
                on:dragleave = on_dragleave
                node_ref=form_element
            >
                <DialogWindow title="Upload image" close_button=false open_status=Arc::clone(&status)>
                    <p class="text-subtext">"Is this correct?"</p>
                    <p class="text-subtext" node_ref=selected_files>""</p>
                    <Button on:click=close_window.clone() {..} type="submit">"Upload"</Button>
                    <Button variant={ButtonVariant::Secondary} on:click={close_window} {..} type="button">
                        "Cancel"
                    </Button>
                </DialogWindow>

                <Icon icon={icondata::LuUpload} width="36" height="36" {..} class="text-text" id="icon_element" />
                <p class="mt-2 text-text pt-2">"Drag and drop an image here"</p>
                <p class="text-sm text-subtext p-2">"or"</p>
                <Button {..} type="button"
                    on:click=move |ev| {
                        ev.prevent_default();

                        let element = input_element.get().unwrap();
                        element.click();
                    }
                >
                    "Browse Files"
                </Button>
                <input type="file" name="file_upload" class="hidden" accept="image/*" on:change=on_change node_ref=input_element />
            </form>
        </div>
    }
}
