use std::ops::Deref;

use yew::prelude::*;

use crate::context::{CalendarState, HeaderButton, LayoutAction, LayoutHandle, LayoutState};

#[derive(Clone)]
pub struct UseLayoutContextHandle {
    inner: LayoutHandle,
}

impl UseLayoutContextHandle {
    pub fn set_login_layout(&self, title: String) {
        let state = LayoutState::builder()
            .calendar(CalendarState::disabled())
            .show_footer(false)
            .title(title)
            .build();
        self.inner.dispatch(LayoutAction::SetLayout(state));
    }
    pub fn set_pub_route_back_button_layout(&self, title: String) {
        let state = LayoutState::builder()
            .calendar(CalendarState::disabled())
            .show_footer(false)
            .left_buttons(vec![HeaderButton::back()])
            .title(title)
            .build();
        self.inner.dispatch(LayoutAction::SetLayout(state));
    }
    pub fn set_app_service_layout(&self, show_footer: bool, title: Option<String>) {
        let state = LayoutState::builder()
            .calendar(CalendarState::disabled())
            .show_footer(show_footer)
            .left_buttons(vec![HeaderButton::back()])
            .title_opt(title)
            .build();
        self.inner.dispatch(LayoutAction::SetLayout(state));
    }
    pub fn set_app_service_edit_layout(
        &self,
        editing: bool,
        title: String,
        form_node_ref: NodeRef,
        edit_onclick: Callback<MouseEvent>,
    ) {
        let (l, r) = if editing {
            (
                HeaderButton::reset(form_node_ref.clone()),
                HeaderButton::submit(form_node_ref),
            )
        } else {
            (HeaderButton::back(), HeaderButton::edit(edit_onclick))
        };
        let state = LayoutState::builder()
            .calendar(CalendarState::disabled())
            .left_buttons(vec![l])
            .right_buttons(vec![r])
            .show_footer(!editing)
            .title(title)
            .build();
        self.inner.dispatch(LayoutAction::SetLayout(state));
    }
    pub fn set_app_layout(&self, right_buttons: Vec<HeaderButton>) {
        let state: LayoutState = LayoutState::builder()
            .show_footer(true)
            .right_buttons(right_buttons)
            .build();
        self.inner.dispatch(LayoutAction::SetLayout(state));
    }
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
