use leptos::prelude::*;

#[component]
pub fn Background() -> impl IntoView {
    view! {
        <div class="bg-hero dark:bg-herod bg-no-repeat bg-cover bg-center h-screen w-full fixed -z-10" />
    }
}
