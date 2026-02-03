use yew::prelude::*;

#[function_component(Background)]
pub fn background() -> Html {
    html! {
        <div
            class="bg-hero dark:bg-herod bg-no-repeat bg-cover bg-center h-screen w-full fixed -z-10"
        />
    }
}
