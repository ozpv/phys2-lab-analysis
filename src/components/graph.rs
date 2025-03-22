use leptos::prelude::*;

#[component]
pub fn Graph() -> impl IntoView {
    view! {
        <div class="w-full md:w-1/2 p-4 rounded-lg shadow-md">
            <h2 class="text-text text-xl mb-4">"Graph"</h2>
            <div class="h-64 flex items-center justify-center">
                <p class="text-text">"Graph will be displayed here"</p>
            </div>
        </div>
    }
}
