use crate::{
    hooks::use_cache_aware_async,
    model::UserInfo,
    routes::PublicRoute,
    services::{
        current,
        requests::{get_token, set_token},
    },
};
use common::error::AppError;
use yew::prelude::*;
use yew_hooks::prelude::use_mount;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_cache_aware_async(current());
    let navigator = use_navigator().unwrap();
    let route = use_route::<PublicRoute>().unwrap();

    {
        /* On startup check if the user is already logged in from local storage. */
        let current_user = current_user.clone();
        let navigator = navigator.clone();
        use_mount(move || {
            if get_token().is_some() {
                log::debug!("Fetching current user info");
                current_user.run();
            } else if !([
                PublicRoute::Login,
                PublicRoute::Register,
                PublicRoute::PasswordReset,
            ]
            .contains(&route)
                || matches!(route, PublicRoute::SharedCharts { id: _ }))
            {
                navigator.push(&PublicRoute::Login);
            }
        });
    }

    {
        /* If local storage has a token either log the user in or show error if couldn't fetch user data. */
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        use_effect_with(current_user, move |current_user| {
            if let Some(user_info) = &current_user.data {
                user_ctx.set(user_info.user.clone());
            }

            if let Some(error) = &current_user.error {
                if let AppError::Unauthorized(_) = error {
                    set_token(None);
                }
                navigator.push(&PublicRoute::Login);
            }
            || ()
        })
    }

    html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    }
}
