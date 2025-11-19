use crate::AppState;
use crate::{css::*, model::auth::*, services::requests::request_api_post};

use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Register() -> impl IntoView {
    let (login_info, set_login_info) = signal(LoginInfo::default());
    let (show_pwd, set_show_pwd) = signal(false);
    let user_login = Action::new_local(|login_info: &LoginInfo| {
        let login_info = LoginInfoWrapper {
            user: login_info.clone(),
        };
        async move { login(login_info).await }
    });

    let navigate = use_navigate();

    let onsubmit = move |ev: SubmitEvent| {
        ev.prevent_default(); // prevent page reload
        user_login.dispatch(login_info.get());
    };

    // let app_state = use_context::<AppState>().unwrap();
    // app_state.header_label.set("Register".into());

    // let error_formatter = move |err| match err {
    //     crate::error::AppError::NotFound => {
    //         Some(Locale::current().login_not_found(&login_info.get().email))
    //     }
    //     _ => None,
    // };

    view! {
        // <BlankPage header_label=Locale::current().login() loading=loading()>
        // <ListErrors error=error() error_formatter=error_formatter />
            <form on:submit=onsubmit>
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
                        <button class=SUBMIT_BTN_CSS type="submit">
                            // <i class="icon-login"></i>{format!(" {}", Locale::current().sign_up())}
                            <i class="icon-login"></i>{"Sign up"}
                        </button>
                    </div>
                    <div class=LINKS_CSS>
                        <A href="/reset-pwd">
                            // {Locale::current().forgot_password()}
                            {"Have an account?"}
                        </A>
                    </div>
                    <div class="fixed bottom-0 justify-between w-full left-0 flex px-4 py-4 text-sm">
                        <A href="/about">
                            // {Locale::current().about()}
                            {"About"}
                        </A>
                        <A href="/help">
                            // {Locale::current().help_and_support()}
                            {"Help and support"}
                        </A>
                    </div>
                </div>
            </form>
        // </BlankPage>
    }
}

async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, AppError> {
    request_api_post("/users/login".to_string(), &login_info).await
}
