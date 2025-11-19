pub mod mobile_layout;

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};
use mobile_layout::MobileLayout;

#[derive(Clone)]
pub struct LayoutContext {
    pub header_title: RwSignal<Option<String>>,
    pub hide_footer: RwSignal<bool>,
    // pub header_buttons: RwSignal<Vec<View>>,
}

impl LayoutContext {
    fn new() -> Self {
        let header_title = RwSignal::new(None);
        let hide_footer = RwSignal::new(false);

        LayoutContext {
            header_title,
            hide_footer,
        }
    }

    fn provide() {
        provide_context(Self::new());
    }

    fn reset(&self) {
        self.header_title.set(None);
        self.hide_footer.set(false);
    }
}

#[component]
pub fn AppLayout() -> impl IntoView {
    // Placeholder for future switch between mobile and table layouts
    let is_mobile = true;
    let loc = use_location();
    let first_load = Rc::new(Cell::new(true));

    // Should be before the call to obtain the context
    LayoutContext::provide();

    let layout = use_context::<LayoutContext>().expect("Could not obtain layout context");

    // Reset layout on navigation
    Effect::new(move || {
        let _ = loc.pathname.get(); // triggers on navigation

        if first_load.get() {
            // Avoid wiping out layout on first load
            log::debug!("Skipping clearing layout context on first load");
            first_load.set(false);
        } else {
            log::debug!("Clearing layout context");
            layout.reset();
        }
    });

    view! {
        {move || {
            if is_mobile {
                view! { <MobileLayout><Outlet/></MobileLayout> }
            } else {
                view! { <MobileLayout><Outlet/></MobileLayout> }
            }
        }}
    }
}
