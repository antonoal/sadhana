use leptos::prelude::*;

use crate::{components::footer::Footer, layouts::LayoutContext};

#[component]
pub fn Charts() -> impl IntoView {
    // let layout = use_context::<LayoutContext>().expect("Could not obtain layout context");

    // layout.header_title.set(Some("Charts".into()));

    view! {
        <div>
            "Charts"
        </div>
        <Footer />
    }
}
