use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_fluent::{move_tr, tr};

use crate::{
    components::{errors::ErrorContext, loading_context::LoadingContext},
    css::*,
    layouts::LayoutContext,
    model::auth::{ConfirmationType, SendConfirmationLink, SendConfirmationLinkWrapper},
    services::requests::request_api_post,
};

#[component]
pub fn Confirmation(confirmation_type: ConfirmationType) -> impl IntoView {
    let loader = LoadingContext::get();
    let layout = LayoutContext::get();
    let error = ErrorContext::get();
    let (email, set_email) = signal(String::default());
    let (email_sent, set_email_sent) = signal(false);
    let (ct, _) = signal(confirmation_type);

    let send_signup_email: Action<_, Result<(), AppError>> =
        loader.start_action(move |email: &String| {
            let email = email.to_owned();
            async move {
                send_confirmation_link(email.to_owned(), ct.get())
                    .await
                    .map(|_| {
                        set_email_sent.set(true);
                    })
            }
        });

    let err_fmt = Callback::new(move |err| match err {
        AppError::UnprocessableEntity(err)
            if err.iter().any(|s| s.ends_with("already exists.")) =>
        {
            Some(tr!("err-user_already_exists", { "email" => email.get() }))
        }
        _ => None,
    });

    Effect::new(move || {
        // TODO: make it error.update(action_result, optional fmt)
        if let Some(Err(e)) = send_signup_email.value().get() {
            error.formatter.set(Some(err_fmt));
            error.errors.set(vec![e]);
        } else {
            error.reset();
        }
    });

    Effect::new(move || {
        let title = match ct.get() {
            ConfirmationType::Registration => tr!("auth_register"),
            ConfirmationType::PasswordReset => tr!("auth_password-reset"),
        };
        layout.set_title(title);
        // layout.set_buttons(
        //     vec![HeaderButton::navigate("icon-chevron-left", "/login")],
        //     vec![],
        // );
        layout.hide_footer();
    });

    let has_error = move || send_signup_email.value().get().is_some_and(|v| v.is_err());
    let labels = move || match ct.get() {
        ConfirmationType::Registration => (tr!("auth_reset-email-sent"), tr!("auth_reset")),
        ConfirmationType::PasswordReset => (tr!("auth_sign-up-email-sent"), tr!("auth_sign-up")),
    };

    view! {
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default(); // prevent page reload
            send_signup_email.dispatch(email.get());
        }>
            <div class=BODY_DIV_CSS >
                <Show
                    when=move|| !email_sent.get() && !has_error()
                    fallback=move || view! {
                        <div class="relative">
                            <label>{ move || labels().0 }</label>
                        </div>
                    }
                >
                    <div class="relative">
                        <input
                            id="email"
                            type="email"
                            placeholder="Email"
                            class=INPUT_CSS
                            prop:value=move || email.get()
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            required = true
                            />
                        <label for="email" class=INPUT_LABEL_CSS >
                            <i class="icon-mail"/>
                            {move || format!(" {}", tr!("auth_email-address"))}
                        </label>
                    </div>
                    <Show when=move||ct.get() == ConfirmationType::Registration >
                        <div class="relative flex justify-between sm:text-base">
                            <a class=LINK_CSS href="/login">
                                { move_tr!("auth_have-an-account") }
                            </a>
                        </div>
                    </Show>
                    <div class="relative">
                        <button class=SUBMIT_BTN_CSS >
                            {move || labels().1}
                        </button>
                    </div>
                </Show>
            </div>
        </form>
    }
}

async fn send_confirmation_link(
    email: String,
    confirmation_type: ConfirmationType,
) -> Result<(), AppError> {
    request_api_post(
        "/users/confirmation",
        &SendConfirmationLinkWrapper::from(SendConfirmationLink::new(email, confirmation_type)),
    )
    .await
}
