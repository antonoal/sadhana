use std::future::Future;

use common::error::AppError;
use yew::prelude::*;
use yew_hooks::{UseAsyncHandle, use_async};

use crate::{context::LoadingContext, hooks::use_errors_ctx};

#[hook]
pub fn use_async_with_error<T, F>(future: F) -> UseAsyncHandle<T, AppError>
where
    F: Future<Output = Result<T, AppError>> + 'static,
    T: Clone + 'static,
{
    let errors = use_errors_ctx();
    let handle = use_async(future);
    let loading = use_context::<LoadingContext>().expect("LoadingContext not found");

    {
        // Errors propagation
        let errors = errors.clone();
        let err = handle.error.clone();

        use_effect_with(err, move |err| {
            if let Some(err) = err.clone() {
                errors.push_error(err);
            }
            || ()
        });
    }

    {
        // Loading indicator update
        let start = loading.start.clone();
        let stop = loading.stop.clone();
        let is_loading = handle.loading;

        use_effect_with(is_loading, move |loading| {
            if *loading {
                start.emit(());
            } else {
                stop.emit(());
            }

            || ()
        });
    }

    handle
}
