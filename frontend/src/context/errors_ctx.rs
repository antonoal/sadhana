use common::error::AppError;
use std::{collections::HashSet, rc::Rc};
use yew::{html::ChildrenProps, prelude::*};
use yew_router::hooks::use_route;

use crate::routes::{AppRoute, PublicRoute};

#[derive(Clone, Default, PartialEq)]
pub struct ErrorsState {
    pub errors: HashSet<AppError>,
}

pub enum ErrorsAction {
    Push(AppError),
    Remove(AppError),
    Reset,
}

impl Reducible for ErrorsState {
    type Action = ErrorsAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new = (*self).clone();

        match action {
            ErrorsAction::Push(error) => {
                new.errors.insert(error);
            }
            ErrorsAction::Remove(error) => {
                new.errors.remove(&error);
            }
            ErrorsAction::Reset => new = Self::default(),
        }

        new.into()
    }
}

pub type ErrorsHandle = UseReducerHandle<ErrorsState>;

#[function_component(ErrorContextProvider)]
pub fn errors_context_provider(props: &ChildrenProps) -> Html {
    let errors = use_reducer(ErrorsState::default);
    let public_route = use_route::<PublicRoute>();
    let app_route = use_route::<AppRoute>();

    {
        let errors = errors.clone();
        use_effect_with((public_route, app_route), move |_| {
            errors.dispatch(ErrorsAction::Reset);
            || ()
        });
    }

    html! {
        <ContextProvider<ErrorsHandle> context={errors}>
            { props.children.clone() }
        </ContextProvider<ErrorsHandle>>
    }
}
