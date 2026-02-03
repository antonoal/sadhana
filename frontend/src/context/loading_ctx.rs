use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use gloo::timers::callback::Timeout;
use yew::{html::ChildrenProps, prelude::*};

#[derive(Clone, PartialEq)]
pub struct LoadingContext {
    pub start: Callback<()>,
    pub stop: Callback<()>,
    pub is_loading_delayed: UseStateHandle<bool>,
}

impl LoadingContext {
    pub fn new(is_loading_delayed: UseStateHandle<bool>) -> Self {
        let loading_count = Rc::new(Cell::new(0usize));
        let timer = Rc::new(RefCell::new(None::<Timeout>));

        let start = {
            let loading_count = loading_count.clone();
            let timer = timer.clone();
            let is_loading_delayed = is_loading_delayed.clone();

            Callback::from(move |_| {
                let count = loading_count.get() + 1;
                loading_count.set(count);

                if count == 1 {
                    if let Some(t) = timer.borrow_mut().take() {
                        t.cancel();
                    }

                    let loading_count = loading_count.clone();
                    let is_loading_delayed = is_loading_delayed.clone();

                    let timeout = Timeout::new(400, move || {
                        if loading_count.get() > 0 {
                            is_loading_delayed.set(true);
                        }
                    });

                    *timer.borrow_mut() = Some(timeout);
                }
            })
        };

        let stop = {
            let loading_count = loading_count.clone();
            let timer = timer.clone();
            let is_loading_delayed = is_loading_delayed.clone();

            Callback::from(move |_| {
                let count = loading_count.get().saturating_sub(1);
                loading_count.set(count);

                if count == 0 {
                    is_loading_delayed.set(false);
                    if let Some(t) = timer.borrow_mut().take() {
                        t.cancel();
                    }
                }
            })
        };

        Self {
            start,
            stop,
            is_loading_delayed,
        }
    }
}

#[function_component(LoadingContextProvider)]
pub fn loading_ctx_provider(props: &ChildrenProps) -> Html {
    let is_loading_delayed = use_state(|| false);
    let loading_context = LoadingContext::new(is_loading_delayed);

    html! {
        <ContextProvider<LoadingContext> context={loading_context}>
            { props.children.clone() }
        </ContextProvider<LoadingContext>>
    }
}
