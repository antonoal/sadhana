use common::error::AppError;
use leptos::prelude::*;
use leptos_fluent::tr;

#[derive(Clone, Debug)]
pub struct ErrorContext {
    pub errors: RwSignal<Vec<AppError>>,
    pub formatter: RwSignal<Option<Callback<AppError, Option<String>>>>,
}

impl ErrorContext {
    pub fn provide() {
        let errors = RwSignal::new(vec![]);
        let formatter = RwSignal::new(None);
        provide_context(Self { errors, formatter });
    }

    pub fn reset(&self) {
        self.errors.set(vec![]);
        self.formatter.set(None);
    }

    pub fn get() -> Self {
        use_context::<Self>().expect("Could not obtain error context")
    }
}

#[component]
pub fn ErrorBanner() -> impl IntoView {
    // TODO: dismissal of errors
    //     use leptos_use::use_timeout_fn;

    // use_timeout_fn(
    //     move || global_error.0.set(None),
    //     5_000.0,
    // );
    let error_ctx = ErrorContext::get();

    let error_msgs = move || {
        error_ctx
            .errors
            .get()
            .into_iter()
            .flat_map(|err| {
                error_ctx
                    .formatter
                    .get()
                    .and_then(|fmt| fmt.run(err.clone()))
                    .map(|s| vec![s])
                    .unwrap_or(default(err))
            })
            .collect::<Vec<_>>()
    };

    view! {
        {move || error_msgs().into_iter().map(|msg| {
            view! {
                <div
                    class="relative rounded-md border py-2 px-2 bg-red-900 bg-opacity-30 border-red-900"
                    role="alert"
                >
                    <p class="text-gray dark:text-zinc-100 left-2">{ msg }</p>
                </div>
            }
        }).collect_view()}
    }
}

fn default(error: AppError) -> Vec<String> {
    match error {
        AppError::UnprocessableEntity(error_info) => error_info.to_owned(),
        AppError::RequestError => vec![tr!("err-request_error")],
        AppError::InternalServerError => vec![tr!("err-internal_error")],
        AppError::Unauthorized(_) => vec![tr!("err-unauthorized")], // TODO: nav.push(&AppRoute::Login); ??
        _ => vec![error.to_string()],
    }
}
