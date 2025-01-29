use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
};

use crate::components::camera::CameraModal;
use crate::components::footer::Footer;
use crate::components::upload::ImageUploadForm;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/phys2-lab-analysis.css"/>

        <Title text="phys2 lab analysis"/>

        <Router>
            <FlatRoutes fallback=NotFound>
                <Route path=path!("") view=HomePage/>
            </FlatRoutes>
            <Footer/>
        </Router>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <h1>"404 Not Found"</h1>
    }
}
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <main>
            <h1>"Welcome"</h1>
            <ImageUploadForm/>
            <CameraModal/>
        </main>
    }
}
