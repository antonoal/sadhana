use crate::layouts::{ButtonAction, ButtonType, HeaderButton, HeaderButtons};
use crate::{css::*, layouts::LayoutContext, model::auth::*, services::requests::request_api_post};

use common::error::AppError;
use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_fluent::{I18n, move_tr, tr};
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

// TODO: move somewhere common where can be used by About link on the Settings page
fn about_url() -> String {
    let i18n = expect_context::<I18n>();
    format!("https://sadhana.pro/{}", i18n.language.get().id)
}

#[component]
pub fn Register() -> impl IntoView {
    let layout = use_context::<LayoutContext>().expect("Could not obtain layout context");
    let (login_info, set_login_info) = signal(LoginInfo::default());
    let (show_pwd, set_show_pwd) = signal(false);
    let user_login = Action::new_local(|login_info: &LoginInfo| {
        let login_info = LoginInfoWrapper {
            user: login_info.clone(),
        };
        async move { login(login_info).await }
    });

    Effect::new(move || {
        layout.set_title(tr!("auth_reg-title"));
        layout.set_buttons(
            vec![HeaderButton::navigate("icon-chevron-left", "/login")],
            vec![],
        );
        layout.hide_footer();
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
                            <i class="icon-mail"></i>{move || format!(" {}", tr!("auth_email"))}
                        </label>
                    </div>
                    <div class="relative">
                        <button class=SUBMIT_BTN_CSS type="submit">
                            <i class="icon-login"></i>{move || format!(" {}", tr!("auth_sign-up"))}
                        </button>
                    </div>
                    <div class=LINKS_CSS>
                        <A href="/reset-pwd">
                            {move_tr!("auth_forgot-password")}
                        </A>
                    </div>
                    <div class="fixed bottom-0 justify-between w-full left-0 flex px-4 py-4 text-sm">
                        <A href=move || about_url()>
                            {move_tr!("auth_about")}
                        </A>
                        <A href="/help">
                            {move_tr!("auth_help-and-support")}
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
