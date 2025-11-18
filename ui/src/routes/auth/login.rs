use crate::services::auth::set_token;
use crate::{css::*, model::auth::*, services::requests::request_api_post};

use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Login(set_user_ctx: WriteSignal<UserInfo>) -> impl IntoView {
    let (login_info, set_login_info) = signal(LoginInfo::default());
    let (show_pwd, set_show_pwd) = signal(false);
    let user_login = Action::new_local(|login_info: &LoginInfo| {
        let login_info = LoginInfoWrapper {
            user: login_info.clone(),
        };
        async move { login(login_info).await }
    });

    let navigate = use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(user_info)) = user_login.value().get() {
            log::debug!("Setting user context");
            set_token(Some(user_info.user.token.clone()));
            set_user_ctx.set(user_info.user.clone());
            log::debug!("Redirecting to the home page");
            navigate("/", Default::default());
        }
    });

    // let error_formatter = move |err| match err {
    //     crate::error::AppError::NotFound => {
    //         Some(Locale::current().login_not_found(&login_info.get().email))
    //     }
    //     _ => None,
    // };

    view! {
        // <BlankPage header_label=Locale::current().login() loading=loading()>
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
                            // <i class="icon-key"></i>{format!(" {}", Locale::current().password())}
                            <i class="icon-key"></i>{" Password"}
                        </label>
                    </div>
                    <div class="relative">
                        <button class=SUBMIT_BTN_CSS type="submit">
                            // <i class="icon-login"></i>{format!(" {}", Locale::current().sign_in())}
                            <i class="icon-login"></i>{"Submit"}
                        </button>
                    </div>
            //         <div class=LINKS_CSS>
            //             <A class=LINK_CSS href={BaseRoute::PasswordReset.to_path()}>
            //                 {Locale::current().forgot_password()}
            //             </A>
            //             <A class=LINK_CSS_NEW_ACC href={BaseRoute::Register.to_path()}>
            //                 {Locale::current().need_an_account()}
            //             </A>
            //         </div>
            //         <div class="fixed bottom-0 justify-between w-full left-0 flex px-4 py-4">
            //             <A class=LINK_SMALL_CSS href={BaseRoute::About.to_path()}>
            //                 {Locale::current().about()}
            //             </A>
            //             <A class=LINK_SMALL_CSS href={BaseRoute::Help.to_path()}>
            //                 {Locale::current().help_and_support()}
            //             </A>
            //         </div>
                </div>
            </form>
        // </BlankPage>
    }
}

async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, AppError> {
    request_api_post("/users/login".to_string(), &login_info).await
}
