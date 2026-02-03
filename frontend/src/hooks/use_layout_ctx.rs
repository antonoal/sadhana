use std::ops::Deref;

use yew::prelude::*;

use crate::context::{HeaderButton, LayoutAction, LayoutHandle};

#[derive(Clone)]
pub struct UseLayoutContextHandle {
    inner: LayoutHandle,
}

impl UseLayoutContextHandle {
    pub fn set_title(&self, t: String) {
        self.inner.dispatch(LayoutAction::SetTitle(t));
    }
    pub fn hide_footer(&self) {
        self.inner.dispatch(LayoutAction::SetShowFooter(false));
    }
    pub fn set_show_footer(&self, v: bool) {
        self.inner.dispatch(LayoutAction::SetShowFooter(v));
    }
    pub fn hide_calendar(&self) {
        self.inner.dispatch(LayoutAction::SetShowCalendar(false));
    }
    pub fn set_show_calendar(&self, v: bool) {
        self.inner.dispatch(LayoutAction::SetShowCalendar(v));
    }
    pub fn set_selected_day_incomplete(&self, v: bool) {
        self.inner
            .dispatch(LayoutAction::SetSelectedDayIncomplete(v));
    }
    pub fn highlight_incomplete(&self) {
        self.inner.dispatch(LayoutAction::HighlightIncomplete);
    }
    pub fn set_header_buttons(&self, left: Vec<HeaderButton>, right: Vec<HeaderButton>) {
        self.inner
            .dispatch(LayoutAction::SetHeaderButtons(left, right));
    }
}

impl Deref for UseLayoutContextHandle {
    type Target = LayoutHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[hook]
pub fn use_layout_ctx() -> UseLayoutContextHandle {
    let inner = use_context().unwrap();

    UseLayoutContextHandle { inner }
}
