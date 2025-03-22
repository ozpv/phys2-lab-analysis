use leptos::prelude::*;

#[component]
pub fn Table() -> impl IntoView {
    let headers = vec!["Header 1", "Header 2", "Header 3"];
    let row1 = vec!["Cell 1", "Cell 2", "Cell 3"];
    let row2 = vec!["Cell 1", "Cell 2", "Cell 3"];

    view! {
         <div class="w-full md:w-1/2 bg-white p-4 rounded-lg shadow-md">
            <h2 class="text-xl text-text mb-4">"Table"</h2>
            <table class="w-full">
                <thead>
                    <tr class="bg-mantle">
                        {
                            headers
                                .into_iter()
                                .map(|header| view! {
                                    <th class="text-text px-4 py-2">{header}</th>
                                })
                                .collect_view()
                        }
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        {
                            row1
                                .into_iter()
                                .map(|name| view! {
                                    <td class="text-text px-4 py-2 border">{name}</td>
                                })
                                .collect_view()
                        }
                    </tr>
                    <tr>
                        {
                           row2
                                .into_iter()
                                .map(|name| view! {
                                    <td class="text-text px-4 py-2 border">{name}</td>
                                })
                                .collect_view()
                        }
                    </tr>
                </tbody>
            </table>
        </div>
    }
}
