use leptos::prelude::*;
use leptos_fluent::tr;
use tw_merge::*;

use crate::layouts::LayoutContext;

#[component]
pub fn Home() -> impl IntoView {
    let layout = use_context::<LayoutContext>().expect("Could not obtain layout context");

    log::debug!("Setting title in the layout context");
    layout.header_title.set(Some("Home page".into()));

    view! {
        <h1>{ tr!("sadhana-pro") }</h1>
        "Home page"
    }
}
