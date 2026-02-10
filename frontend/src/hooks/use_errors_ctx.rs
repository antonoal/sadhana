use std::ops::Deref;

use common::error::AppError;
use yew::prelude::*;

use crate::context::{ErrorsAction, ErrorsHandle};

#[derive(Clone)]
pub struct UseErrorsContextHandle {
    inner: ErrorsHandle,
}

impl UseErrorsContextHandle {
    pub fn push_error(&self, error: AppError) {
        self.inner.dispatch(ErrorsAction::Push(error));
    }
    pub fn set_formatter(&self, fmt: Callback<AppError, Option<String>>) {
        self.inner.dispatch(ErrorsAction::SetFormatter(fmt));
    }
    pub fn reset(&self) {
        self.inner.dispatch(ErrorsAction::Reset);
    }
}

impl Deref for UseErrorsContextHandle {
    type Target = ErrorsHandle;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[hook]
pub fn use_errors_ctx() -> UseErrorsContextHandle {
    let inner = use_context().unwrap();

    UseErrorsContextHandle { inner }
}
