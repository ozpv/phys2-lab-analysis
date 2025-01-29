use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-black py-9">
            <span class="block py-1.5 justify-center text-center text-xs text-white font-sans">"Copyleft (ɔ) 2025 "
                <a href="https://ozpv.dev/" class="hover:underline hover:text-cyan-300">"ozpv"</a>
                ". All Wrongs Reserved."
            </span>
        </footer>
    }
}
