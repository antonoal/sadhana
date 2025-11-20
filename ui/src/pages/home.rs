use leptos::prelude::*;
use leptos_fluent::tr;
use tw_merge::*;

use crate::{components::loading_context::LoadingContext, layouts::LayoutContext};

#[component]
pub fn Home() -> impl IntoView {
    let layout = use_context::<LayoutContext>().expect("Could not obtain layout context");
    let loader = use_context::<LoadingContext>().expect("Could not obtain loading context");

    (loader.start)();

    log::debug!("Setting title in the layout context");
    layout.header_title.set(Some("Home page".into()));

    view! {
        <h1>{ tr!("sadhana-pro") }</h1>
        "Home page"
    }
}
