use std::{sync::Arc, time::Duration};

use leptos::prelude::*;

#[derive(Clone)]
pub struct LoadingContext {
    pub start: Arc<dyn Fn() + Send + Sync>,
    pub stop: Arc<dyn Fn() + Send + Sync>,
    pub is_loading_delayed: RwSignal<bool>,
}

impl LoadingContext {
    pub fn provide() {
        let loading_count = RwSignal::new(0usize);
        let is_loading_delayed = RwSignal::new(false);
        let loading_timer = RwSignal::new(None::<TimeoutHandle>);

        let start = Arc::new({
            move || {
                // Increment active tasks
                loading_count.update(|n| *n += 1);

                // If this is the first task, start the delay timer
                if loading_count.get() == 1 {
                    // Clear any existing timer
                    if let Some(t) = loading_timer.get() {
                        t.clear();
                    }

                    // Start a 400ms delay before showing spinner
                    let handle = set_timeout_with_handle(
                        move || {
                            if loading_count.get() > 0 {
                                is_loading_delayed.set(true);
                            }
                        },
                        Duration::from_millis(400),
                    );

                    loading_timer.set(handle.ok());
                }
            }
        });

        let stop = Arc::new({
            move || {
                loading_count.update(|n| {
                    if *n > 0 {
                        *n -= 1;
                    }
                });

                // If no more tasks, hide spinner immediately
                if loading_count.get() == 0 {
                    is_loading_delayed.set(false);

                    // Clear timer if still pending
                    if let Some(t) = loading_timer.get() {
                        t.clear();
                    }
                    loading_timer.set(None);
                }
            }
        });

        provide_context(Self {
            start,
            stop,
            is_loading_delayed,
        });
    }

    pub fn start_action<I, O, F, Fu>(&self, action_fn: F) -> Action<I, O>
    where
        I: 'static,
        O: 'static,
        F: Fn(&I) -> Fu + 'static,
        Fu: Future<Output = O> + 'static,
    {
        let start = self.start.clone();
        let stop = self.stop.clone();
        Action::new_local(move |v| {
            start();
            let res = action_fn(v);
            stop();
            res
        })
    }

    pub fn start_resource<T, F>(&self, f: impl Fn() -> F + 'static) -> LocalResource<T>
    where
        T: 'static,
        F: Future<Output = T> + 'static,
    {
        todo!()
    }

    pub fn get() -> Self {
        use_context::<Self>().expect("Could not obtain loading context")
    }
}
