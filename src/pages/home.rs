use leptos::prelude::*;

use crate::components::buttons::{Button, ButtonSize, ButtonVariant, IntoButtonIcon};
use crate::components::camera::CameraModal;
use crate::components::dialog::DialogWindow;
use crate::components::upload::ImageUploadForm;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <ImageUploadForm/>
            <CameraModal/>
            <DialogWindow title="Hello, World!">
                "Inner"
            </DialogWindow>
            <Button variant=ButtonVariant::Secondary size=ButtonSize::Medium icon=icondata::BiGraphql.into_left()>"Hello"</Button>
        </main>
    }
}
