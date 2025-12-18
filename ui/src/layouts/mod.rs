pub mod mobile_layout;

use std::{cell::Cell, rc::Rc};

use leptos::prelude::*;
use leptos_router::{components::Outlet, hooks::use_location};
use mobile_layout::MobileLayout;

#[derive(Clone)]
pub struct LayoutContext {
    pub header_title: RwSignal<Option<String>>,
    pub hide_footer: RwSignal<bool>,
    pub header_buttons: RwSignal<HeaderButtons>,
}

impl LayoutContext {
    pub fn set_title<S: Into<String>>(&self, value: S) {
        self.header_title.write_only().set(Some(value.into()));
    }

    pub fn set_buttons(&self, left: Vec<HeaderButton>, right: Vec<HeaderButton>) {
        self.header_buttons
            .write_only()
            .set(HeaderButtons { left, right });
    }

    pub fn hide_footer(&self) {
        self.hide_footer.write_only().set(true);
    }
}

#[derive(Clone)]
pub struct HeaderButtons {
    pub left: Vec<HeaderButton>,
    pub right: Vec<HeaderButton>,
}

impl HeaderButtons {
    pub fn new() -> Self {
        Self {
            left: vec![],
            right: vec![],
        }
    }
}

#[derive(Clone)]
pub struct HeaderButton {
    pub icon: Option<String>,
    pub label: Option<String>,
    pub action: ButtonAction,
    pub btn_type: ButtonType,
}

impl HeaderButton {
    pub fn navigate<S: Into<String>>(icon: S, url: S) -> Self {
        HeaderButton {
            icon: Some(icon.into()),
            label: None,
            action: ButtonAction::Navigate(url.into()),
            btn_type: ButtonType::Button,
        }
    }
}

#[derive(Clone)]
pub enum ButtonAction {
    Cb(Callback<()>),
    Navigate(String),
    // CtxMenu(Vec<CtxMenuEntry>),
}

#[derive(Clone)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}
impl ButtonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

impl LayoutContext {
    fn new() -> Self {
        let header_title = RwSignal::new(None);
        let hide_footer = RwSignal::new(false);
        let header_buttons = RwSignal::new(HeaderButtons::new());

        LayoutContext {
            header_title,
            hide_footer,
            header_buttons,
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
