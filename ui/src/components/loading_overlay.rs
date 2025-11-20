use leptos::prelude::*;

use crate::components::loading_context::LoadingContext;

#[component]
pub fn LoadingOverlay() -> impl IntoView {
    let ctx = use_context::<LoadingContext>().unwrap();

    view! {
        <Show when=move || ctx.is_loading_delayed.get()>
            <div class="bg-gray-500 bg-opacity-50 fixed left-0 top-0 z-10 h-full w-full overflow-hidden flex">
                <div class="loader ease-linear rounded-full border-4 border-t-4 border-gray-200 h-10 w-10 m-auto"/>
            </div>
        </Show>
    }
}
