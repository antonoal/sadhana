use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_fluent::tr;
use leptos_router::components::Redirect;
use serde::{Deserialize, Serialize};

use crate::{
    components::{errors::ErrorContext, loading_context::LoadingContext},
    css::*,
    layouts::LayoutContext,
    services::requests::{request_api_get, request_api_put},
};

#[derive(Deserialize, Clone, Debug)]
struct SignupLinkDetailsWrapper {
    confirmation: Confirmation,
}

#[derive(Deserialize, Clone, Debug)]
struct Confirmation {
    id: String,
    email: String,
    expires_at: String,
}

#[derive(Serialize, Debug)]
struct ResetPassword {
    confirmation_id: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct ResetPasswordWrapper {
    data: ResetPassword,
}

#[component]
pub fn PwdReset(id: String) -> impl IntoView {
    let loader = LoadingContext::get();
    let layout = LayoutContext::get();
    let error = ErrorContext::get();

    let (pwd, set_pwd) = signal(String::default());
    let (finished, set_finished) = signal(false);
    let (confirmation, set_confirmation) = signal(None::<SignupLinkDetailsWrapper>);

    let email = {
        let id = id.clone();
        loader.start_resource(move || get_signup_link_details(id.clone()))
    };

    // Fetch confirmation details once on mount
    // {
    //     let id = id.clone();
    //     let set_confirmation = set_confirmation.clone();
    //     let error = error.clone();
    //     spawn_local(async move {
    //         match request_api_get::<SignupLinkDetailsWrapper>(&format!(
    //             "/users/confirmation/{}",
    //             id
    //         ))
    //         .await
    //         {
    //             Ok(data) => set_confirmation.set(Some(data)),
    //             Err(e) => error.errors.set(vec![e]),
    //         }
    //     });
    // }

    // Reset password action
    let reset_pwd: Action<_, Result<(), AppError>> = loader.start_action(move |pw: &String| {
        let id = email
            .get()
            .and_then(|d| d.ok())
            .map(|d| d.confirmation.id)
            .unwrap_or_default()
            .clone();
        let pw = pw.to_owned();
        async move {
            reset_pwd(id.clone(), pw)
                .await
                .map(|_| set_finished.set(true))
        }
    });

    // Update error context when action fails
    Effect::new(move || {
        if let Some(Err(e)) = reset_pwd.value().get() {
            error.errors.set(vec![e]);
        } else {
            error.reset();
        }
    });

    Effect::new(move || {
        layout.set_title(tr!("auth_password-reset"));
        layout.hide_footer();
    });

    view! {
        <Show when=move || !finished.get()
            fallback=move || view! {
                <Redirect path="/login" />
            }>
            <form on:submit=move |ev: SubmitEvent| {
                ev.prevent_default();
                reset_pwd.dispatch(pwd.get());
            }>
                <div class=BODY_DIV_CSS>
                    <Suspense fallback=move || view! { <p>{"Loading..."}</p> }>
                        {move || email.get().map(|email| view! {
                            <div>
                                <div class="relative">
                                    <input
                                        id="password"
                                        type="password"
                                        placeholder="Password"
                                        class=INPUT_CSS
                                        prop:value=move || pwd.get()
                                        on:input=move |ev| set_pwd.set(event_target_value(&ev))
                                        required=true
                                    />
                                    <label for="password" class=INPUT_LABEL_CSS>
                                        <i class="icon-key" />
                                        {move || format!(" {}", tr!("auth-password"))}
                                    </label>
                                </div>
                                <div class="relative">
                                    <button class=SUBMIT_BTN_CSS type="submit">{ move || tr!("act-save") }</button>
                                </div>
                            </div>
                        })}
                    </Suspense>
                </div>
            </form>
        </Show>
    }
}

async fn get_signup_link_details(id: String) -> Result<SignupLinkDetailsWrapper, AppError> {
    request_api_get(&format!("/users/confirmation/{}", id)).await
}

async fn reset_pwd(confirmation_id: String, password: String) -> Result<(), AppError> {
    request_api_put(
        "/password-reset",
        &ResetPasswordWrapper {
            data: ResetPassword {
                confirmation_id,
                password,
            },
        },
    )
    .await
}
