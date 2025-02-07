use leptos::{html, prelude::*};
use std::sync::Arc;
use web_sys::MouseEvent;

use crate::components::buttons::{Button, ButtonSize, ButtonVariant};

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
        <div
            tabindex="-1"
            class="fixed bg-crust bg-opacity-70 max-h-full w-full h-full top-0 left-0 z-50 overflow-x-hidden overflow-y-auto p-4 md:inset-0"
            class:hidden=status
            node_ref=outer_dialog
        >
            {
                if close_button {
                    // TODO: Add an X icon here
                    view! { <Button variant=ButtonVariant::Outlined size={ButtonSize::Large} {..} on:click=update_status>"Close"</Button> }.into_any()
                } else {
                    ().into_any()
                }
            }
            {children()}
        </div>
    }
}
