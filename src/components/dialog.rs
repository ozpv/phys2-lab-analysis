use leptos::{html, prelude::*};
use std::sync::Arc;
use web_sys::MouseEvent;

use crate::components::buttons::{Button, ButtonSize};

/// spawns a floating window above all other elements
#[component]
pub fn DialogWindow(
    /// title of the dialog window
    #[prop(into)]
    title: String,
    /// if the dialog includes a close button at the top right
    #[prop(default = true)]
    close_button: bool,
    /// function to run in the browser on dialog close
    #[prop(optional)]
    on_close: Option<&'static dyn Fn()>,
    /// custom open/closed status that syncs with other elements outside
    #[prop(into, optional)]
    open_status: Option<Arc<RwSignal<bool>>>,
    /// the body of the dialog window
    children: Children,
) -> impl IntoView {
    let outer_dialog = NodeRef::<html::Div>::new();

    let open_status = open_status.unwrap_or_else(|| RwSignal::new(true).into());

    let binding = Arc::clone(&open_status);
    let update_status = move |_: MouseEvent| {
        binding.update(|status| *status = !*status);

        // run on close function in the browser if passed
        if let Some(on_close) = on_close {
            if !binding.get() {
                on_close();
            }
        }
    };

    let binding = Arc::clone(&open_status);
    let status = move || !binding.get();

    view! {
        <div tabindex="-1"
            class="fixed inset-0 flex flex-row justify-center items-center bg-crust bg-opacity-60 max-h-full
                w-full h-full z-50 overflow-x-hidden overflow-y-auto"
            class:hidden=status
            node_ref=outer_dialog
        >
            <div class="flex justify-between items-center mb-4">
                <h2 class="text-xl text-text">{title}</h2>
            </div>
            <div class="mb-4">
                {children()}
            </div>
            <div class="flex justify-end">
                {
                    if close_button {
                        // TODO: Add an X icon here
                        view! { <Button size={ButtonSize::Medium} {..} on:click=update_status>"Close"</Button> }.into_any()
                    } else {
                        ().into_any()
                    }
                }
            </div>
        </div>
    }
}
