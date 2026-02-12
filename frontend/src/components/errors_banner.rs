use crate::{
    hooks::use_errors_ctx,
    routes::{AppRoute, PublicRoute},
    tr,
};
use common::error::AppError;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ErrorsBanner)]
pub fn errors_banner() -> Html {
    let errors = use_errors_ctx();
    let pub_route = use_route::<PublicRoute>();
    let app_route = use_route::<AppRoute>();

    html! {
        { for errors.errors.iter().flat_map(|err| {
            let on_close = {
                let errors = errors.clone();
                let err = err.clone();
                Callback::from(move |_| {
                    errors.remove_error(err.clone());
                })
            };
            fmt(err, pub_route.as_ref(), app_route.as_ref()).into_iter().map(move |msg| html! {
                <div
                    class="relative rounded-md border py-2 px-2 bg-red-900 bg-opacity-30 border-red-900"
                    role="alert"
                >
                    <button
                        type="button"
                        class="absolute right-2 top-2 text-sm text-gray dark:text-zinc-100"
                        onclick={on_close.clone()}
                    >
                        { "X" }
                    </button>
                    <p class="text-gray dark:text-zinc-100 left-2">{ msg }</p>
                </div>
            })
        }) }
    }
}

fn fmt(
    error: &AppError,
    pub_route: Option<&PublicRoute>,
    app_route: Option<&AppRoute>,
) -> Vec<String> {
    match error {
        AppError::NotFound => {
            let msg = match pub_route {
                Some(PublicRoute::Login) => tr!(login_not_found),
                Some(PublicRoute::PasswordReset) => tr!(invalid_reset_link),
                _ => tr!(err_not_found),
            };
            vec![msg]
        }
        AppError::UnprocessableEntity(err) => match (pub_route, app_route) {
            (Some(PublicRoute::Register), _) => vec![tr!(user_already_exists)],
            (_, Some(AppRoute::NewUserPractice | AppRoute::NewUserPracticeWithName { .. }))
                if err.iter().any(|s| s.contains("already exists")) =>
            {
                vec![tr!(practice_already_exists)]
            }
            _ => err.clone(),
        },
        AppError::Unauthorized(_) => vec![tr!(unauthorized_error)],
        AppError::Forbidden(_) => vec![tr!(internal_server_error)],
        AppError::InternalServerError => vec![tr!(internal_server_error)],
        AppError::DeserializeError => vec![tr!(internal_server_error)],
        AppError::RequestError => vec![tr!(request_error)],
    }
}
