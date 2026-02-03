use common::error::AppError;
use std::rc::Rc;
use yew::{html::ChildrenProps, prelude::*};

#[derive(Clone, Default, PartialEq)]
pub struct ErrorsState {
    pub errors: Vec<AppError>,
    pub formatter: Option<Callback<AppError, Option<String>>>,
}

pub enum ErrorsAction {
    Push(AppError),
    SetFormatter(Callback<AppError, Option<String>>),
    Reset,
}

impl Reducible for ErrorsState {
    type Action = ErrorsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();

        match action {
            ErrorsAction::Push(error) => new.errors.push(error),
            ErrorsAction::SetFormatter(fmt) => new.formatter = Some(fmt),
            ErrorsAction::Reset => new = Self::default(),
        }

        new.into()
    }
}

pub type ErrorsHandle = UseReducerHandle<ErrorsState>;

#[function_component(ErrorContextProvider)]
pub fn errors_context_provider(props: &ChildrenProps) -> Html {
    // TODO: dismiss errors on timeout or on user action or in the hook when error is no longer present?
    let errors = use_reducer(ErrorsState::default);

    log::debug!("Providing Errors context");

    html! {
        <ContextProvider<ErrorsHandle> context={errors}>
            { props.children.clone() }
        </ContextProvider<ErrorsHandle>>
    }
}
