use super::about_url;
use crate::components::errors::ErrorContext;
use crate::components::loading_context::LoadingContext;
use crate::services::auth::set_token;
use crate::{css::*, model::auth::*, services::requests::request_api_post};

use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_fluent::{move_tr, tr};
use leptos_router::hooks::use_navigate;

#[component]
pub fn Login(set_user_ctx: WriteSignal<UserInfo>) -> impl IntoView {
    let loader = LoadingContext::get();
    let errors = ErrorContext::get();

    let (login_info, set_login_info) = signal(LoginInfo::default());
    let (show_pwd, set_show_pwd) = signal(false);

    let user_login = loader.start_action(move |login_info: &LoginInfo| {
        let user = login_info.clone();
        async move { login(user).await }
    });

    let navigate = use_navigate();

    Effect::new(move || {
        if let Some(Ok(user_info)) = user_login.value().get() {
            log::debug!("Setting user context");
            set_token(Some(user_info.user.token.clone()));
            set_user_ctx.set(user_info.user.clone());
            log::debug!("Redirecting to the home page");
            navigate("/", Default::default());
        }

        if let Some(Err(err)) = user_login.value().get() {
            errors.errors.set(vec![err]);
        }
    });

    // let error_formatter = move |err| match err {
    //     crate::error::AppError::NotFound => {
    //         Some(Locale::current().login_not_found(&login_info.get().email))
    //     }
    //     _ => None,
    // };

    view! {
        // <ListErrors error=error() error_formatter=error_formatter />
            <form on:submit=move |ev: SubmitEvent| {
                ev.prevent_default(); // prevent page reload
                user_login.dispatch(login_info.get());
            }>
                <div class=BODY_DIV_CSS>
                    <div class="relative">
                        <input
                            type="email"
                            id="email"
                            placeholder="Email"
                            prop:value=move || login_info.get().email
                            on:input=move |ev| set_login_info.update(|info| info.email = event_target_value(&ev))
                            class=INPUT_CSS
                            required=true
                        />
                        <label for="email" class=INPUT_LABEL_CSS>
                            // <i class="icon-mail"></i>{format!(" {}", Locale::current().email_address())}
                            <i class="icon-mail"></i>{" Email address"}
                        </label>
                    </div>
                    <div class="relative">
                        <input
                            autocomplete="off"
                            id="password"
                            type=move || if show_pwd.get() { "text" } else { "password" }
                            placeholder="Password"
                            class=INPUT_CSS
                            prop:value=move || login_info.get().password
                            on:input=move |ev| set_login_info.update(|info| info.password = event_target_value(&ev))
                            required=true
                        />
                        <div class="absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5">
                            <i
                                class=move || if show_pwd.get() { "icon-eye-cross" } else { "icon-eye" }
                                on:click=move |_| set_show_pwd.update(|v| *v = !*v)
                            />
                        </div>
                        <label for="password" class=INPUT_LABEL_CSS>
                            <i class="icon-key" />
                            {move || format!(" {}", tr!("auth-password"))}
                        </label>
                    </div>
                    <div class="relative">
                        <button class=SUBMIT_BTN_CSS type="submit">
                            <i class="icon-login" />
                            {move || format!(" {}", tr!("auth-sign_in"))}
                        </button>
                    </div>
                    <div class=LINKS_CSS>
                        <a class=LINK_CSS href="/reset">
                            {move_tr!("auth-forgot_password")}
                        </a>
                        <a class=LINK_CSS_NEW_ACC href="/register">
                            {move_tr!("auth-need_account")}
                        </a>
                    </div>
                    <div class="fixed bottom-0 justify-between w-full left-0 flex px-4 py-4">
                        <a class=LINK_SMALL_CSS href=about_url>
                            {move_tr!("auth-about")}
                        </a>
                        <a class=LINK_SMALL_CSS href="/help">
                            {move_tr!("auth-help_and_support")}
                        </a>
                    </div>
                </div>
            </form>
    }
}

async fn login(login_info: LoginInfo) -> Result<UserInfoWrapper, AppError> {
    request_api_post("/users/login", &LoginInfoWrapper::from(login_info)).await
}
