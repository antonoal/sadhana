use std::rc::Rc;
use strum::Display;
use web_sys::HtmlFormElement;
use yew::{html::ChildrenProps, html::onclick::Event, prelude::*};
use yew_hooks::use_location;

use crate::{routes::AppRoute, tr};

#[derive(Debug, Clone, PartialEq)]
pub struct CtxMenuEntry {
    pub label: String,
    pub icon_css: Option<String>,
    pub action: Action,
}
impl CtxMenuEntry {
    pub fn action<S: Into<String>>(onclick: Callback<Event>, icon_css: S, label: S) -> Self {
        Self {
            label: label.into(),
            icon_css: Some(icon_css.into()),
            action: Action::Cb(onclick),
        }
    }

    pub fn link<S: Into<String>>(route: AppRoute, icon_css: S, label: S) -> Self {
        Self {
            label: label.into(),
            icon_css: Some(icon_css.into()),
            action: Action::Redirect(route),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Cb(Callback<Event>),
    Redirect(AppRoute),
    NavBack,
    CtxMenu(Vec<CtxMenuEntry>),
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeaderButton {
    pub label: Option<String>,
    pub icon_css: Option<String>,
    pub action: Action,
    pub btn_type: ButtonType,
}

impl HeaderButton {
    pub fn new_cb<S: Into<String>>(
        label: S,
        onclick: Callback<Event>,
        icon_css: Option<String>,
        btn_type: ButtonType,
    ) -> Self {
        Self {
            label: Some(label.into()).filter(|s| !s.is_empty()),
            icon_css,
            action: Action::Cb(onclick),
            btn_type,
        }
    }

    pub fn new_redirect<S: Into<String>>(
        label: S,
        route: AppRoute,
        icon_css: Option<String>,
        btn_type: ButtonType,
    ) -> Self {
        Self {
            label: Some(label.into()),
            icon_css,
            action: Action::Redirect(route),
            btn_type,
        }
    }

    pub fn new_icon_cb<S: Into<String>>(
        onclick: Callback<Event>,
        icon_css: S,
        btn_type: ButtonType,
    ) -> Self {
        Self {
            label: None,
            icon_css: Some(icon_css.into()),
            action: Action::Cb(onclick),
            btn_type,
        }
    }

    pub fn new_icon_redirect<S: Into<String>>(route: AppRoute, icon_css: S) -> Self {
        Self {
            label: None,
            icon_css: Some(icon_css.into()),
            action: Action::Redirect(route),
            btn_type: ButtonType::Button,
        }
    }

    pub fn edit(onclick: Callback<Event>) -> Self {
        Self::new_cb("", onclick, Some("icon-edit".into()), ButtonType::Button)
    }

    pub fn done(redirect_to: AppRoute) -> Self {
        Self::new_redirect(tr!(done), redirect_to, None, ButtonType::Button)
    }

    pub fn submit<S: Into<String>>(label: S, form_ref: NodeRef) -> Self {
        Self::new_cb(
            label,
            Callback::from(move |_| {
                if let Some(form) = form_ref.cast::<HtmlFormElement>() {
                    let _ = form.request_submit();
                }
            }),
            None,
            ButtonType::Submit,
        )
    }

    pub fn reset<S: Into<String>>(label: S, form_ref: NodeRef) -> Self {
        Self::new_cb(
            label.into(),
            Callback::from(move |_| {
                if let Some(form) = form_ref.cast::<HtmlFormElement>() {
                    form.reset();
                }
            }),
            None,
            ButtonType::Button,
        )
    }

    pub fn blank() -> Self {
        Self {
            label: None,
            action: Action::Cb(Callback::default()),
            icon_css: None,
            btn_type: ButtonType::Button,
        }
    }

    pub fn back() -> Self {
        Self {
            label: None,
            icon_css: Some("icon-chevron-left".into()),
            action: Action::NavBack,
            btn_type: ButtonType::Button,
        }
    }

    pub fn back_to(to: AppRoute) -> Self {
        Self::new_icon_redirect(to, "icon-chevron-left")
    }

    pub fn month_calendar(onclick: Callback<Event>) -> Self {
        Self::new_icon_cb(onclick, "icon-calendar", ButtonType::Button)
    }

    pub fn ctx_menu<S: Into<String>>(icon_css: S, entries: Vec<CtxMenuEntry>) -> Self {
        Self {
            label: None,
            icon_css: Some(icon_css.into()),
            action: Action::CtxMenu(entries),
            btn_type: ButtonType::Button,
        }
    }
}

/// Calendar layout settings
#[derive(Clone, PartialEq)]
pub struct CalendarState {
    pub show: bool,
    /// Overrides incompleteness of the selected day.
    /// Since calendar pulls incomplete days only once this field is updated
    /// when the day becomes complete on the Input page.
    pub selected_day_incomplete: Option<bool>,
    pub highlight_incomplete: bool,
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            show: true,
            selected_day_incomplete: None,
            highlight_incomplete: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LayoutState {
    pub title: Option<String>,
    pub show_footer: bool,
    pub calendar: CalendarState,
    pub left_buttons: Vec<HeaderButton>,
    pub right_buttons: Vec<HeaderButton>,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            title: Default::default(),
            show_footer: true,
            calendar: Default::default(),
            left_buttons: Default::default(),
            right_buttons: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum LayoutAction {
    SetTitle(String),
    SetShowFooter(bool),
    SetShowCalendar(bool),
    SetSelectedDayIncomplete(bool),
    HighlightIncomplete,
    SetHeaderButtons(Vec<HeaderButton>, Vec<HeaderButton>),
    Reset,
}

impl Reducible for LayoutState {
    type Action = LayoutAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();

        match action {
            LayoutAction::SetTitle(t) => new.title = (!t.is_empty()).then_some(t),
            LayoutAction::SetShowCalendar(v) => {
                new.calendar = CalendarState {
                    show: v,
                    ..Default::default()
                }
            }
            LayoutAction::SetSelectedDayIncomplete(v) => {
                new.calendar = CalendarState {
                    selected_day_incomplete: Some(v),
                    ..self.calendar
                }
            }
            LayoutAction::HighlightIncomplete => {
                new.calendar = CalendarState {
                    highlight_incomplete: true,
                    ..self.calendar
                }
            }
            LayoutAction::SetHeaderButtons(left, right) => {
                new.left_buttons = left;
                new.right_buttons = right;
            }
            LayoutAction::SetShowFooter(v) => new.show_footer = v,
            LayoutAction::Reset => new = Self::default(),
        }

        new.into()
    }
}

pub type LayoutHandle = UseReducerHandle<LayoutState>;

#[function_component(LayoutStateProvider)]
pub fn layout_state_provider(props: &ChildrenProps) -> Html {
    let layout = use_reducer(LayoutState::default);
    let loc = use_location();
    let pathname = loc.pathname.clone();
    let prev_path = use_mut_ref(|| pathname.clone());

    {
        let layout = layout.clone();
        let prev_path = prev_path.clone();
        use_effect_with(pathname, move |path| {
            if *prev_path.borrow() != *path {
                layout.dispatch(LayoutAction::Reset);
                *prev_path.borrow_mut() = path.clone();
            }
            || ()
        });
    }

    html! {
        <ContextProvider<LayoutHandle> context={layout}>
            { props.children.clone() }
        </ContextProvider<LayoutHandle>>
    }
}
