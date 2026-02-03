use crate::{hooks::use_errors_ctx, tr};
use common::error::AppError;
use yew::prelude::*;

#[function_component(ErrorsBanner)]
pub fn errors_banner() -> Html {
    let errors = use_errors_ctx();

    let error_msgs = move || {
        errors
            .errors
            .iter()
            .flat_map(|err| {
                errors
                    .formatter
                    .as_ref()
                    .and_then(|fmt| fmt.emit(err.clone()))
                    .map(|s| vec![s])
                    .unwrap_or(default(err))
            })
            .collect::<Vec<_>>()
    };

    html! {
        { for error_msgs().into_iter().map(|msg| {
            html! {
                <div
                    class="relative rounded-md border py-2 px-2 bg-red-900 bg-opacity-30 border-red-900"
                    role="alert"
                >
                    <p class="text-gray dark:text-zinc-100 left-2">{ msg }</p>
                </div>
            }
        }) }
    }
}

fn default(error: &AppError) -> Vec<String> {
    match error {
        AppError::UnprocessableEntity(error_info) => error_info.to_owned(),
        // AppError::RequestError => vec![tr!("err-request_error")],
        // AppError::InternalServerError => vec![tr!("err-internal_error")],
        // AppError::Unauthorized(_) => vec![tr!("err-unauthorized")], // TODO: nav.push(&AppRoute::Login); ??
        _ => vec![error.to_string()],
    }
}
