use leptos::prelude::*;

use crate::components::camera::CameraModal;
use crate::components::dialog::DialogWindow;
use crate::components::graph::Graph;
use crate::components::table::Table;
use crate::components::upload::ImageUploadForm;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <main>
            <ImageUploadForm/>
            <CameraModal/>
            <DialogWindow title="Hello, World!">
                "Inner body"
            </DialogWindow>
            <div class="container mx-auto">
                <div class="flex flex-col md:flex-row gap-4">
                    <Table/>
                    <Graph/>
                </div>
            </div>
        </main>
    }
}
