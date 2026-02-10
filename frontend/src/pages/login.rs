use common::error::AppError;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_bool_toggle, use_mount};
use yew_router::prelude::*;

use crate::{
    PublicRoute,
    css::*,
    hooks::{use_async_with_error, use_errors_ctx, use_layout_ctx, use_user_ctx},
    i18n::*,
    model::*,
    services, tr,
};

#[function_component(Login)]
pub fn login() -> Html {
    let layout = use_layout_ctx();
    let errors = use_errors_ctx();
    let user_ctx = use_user_ctx();
    let login_info = use_state(LoginInfo::default);
    let show_pwd = use_bool_toggle(false);

    let user_login = {
        let login_info = login_info.clone();
        use_async_with_error(async move {
            let request = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            services::login(&request).await
        })
    };

    let error_formatter = {
        let login_info = login_info.clone();
        Callback::from(move |err| match err {
            AppError::NotFound => Some(tr!(login_not_found, Email(&login_info.email))),
            _ => None,
        })
    };

    {
        let layout = layout.clone();
        let errors = errors.clone();
        let fmt = error_formatter.clone();
        use_mount(move || {
            layout.set_login_layout(tr!(login));
            errors.set_formatter(fmt);
        });
    }

    /* Hook into changes of user_login */
    use_effect_with(user_login.clone(), move |user_login| {
        if let Some(user_info) = &user_login.data {
            user_ctx.login(user_info.user.clone());
        }
        || ()
    });

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            user_login.run();
        })
    };

    let oninput_email = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    let toggle_show_pwd_onclick = {
        let show_pwd = show_pwd.clone();
        Callback::from(move |_| {
            show_pwd.toggle();
        })
    };

    html! {
        <form {onsubmit}>
            <div class={BODY_DIV_CSS}>
                <div class="relative">
                    <input
                        type="email"
                        id="email"
                        placeholder="Email"
                        value={login_info.email.clone()}
                        oninput={oninput_email}
                        class={INPUT_CSS}
                        required=true
                    />
                    <label for="email" class={INPUT_LABEL_CSS}>
                        <i class="icon-mail" />
                        { format!(" {}", tr!(email_address)) }
                    </label>
                </div>
                <div class="relative">
                    <input
                        autocomplete="off"
                        id="password"
                        type={if *show_pwd {"text"} else {"password"}}
                        placeholder="Password"
                        class={INPUT_CSS}
                        value={login_info.password.clone()}
                        oninput={oninput_password}
                        required=true
                    />
                    <div
                        class="absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5"
                    >
                        <i
                            class={if *show_pwd {"icon-eye-cross"} else {"icon-eye"}}
                            onclick={toggle_show_pwd_onclick}
                        />
                    </div>
                    <label for="password" class={INPUT_LABEL_CSS}>
                        <i class="icon-key" />
                        { format!(" {}", tr!(password)) }
                    </label>
                </div>
                <div class="relative">
                    <button class={SUBMIT_BTN_CSS}>
                        <i class="icon-login" />
                        { format!(" {}", tr!(sign_in)) }
                    </button>
                </div>
                <div class={LINKS_CSS}>
                    <Link<PublicRoute> classes={LINK_CSS} to={PublicRoute::PasswordReset}>
                        { tr!(forgot_password) }
                    </Link<PublicRoute>>
                    <Link<PublicRoute> classes={LINK_CSS_NEW_ACC} to={PublicRoute::Register}>
                        { tr!(need_an_account) }
                    </Link<PublicRoute>>
                </div>
                <div class="fixed bottom-0 justify-between w-full left-0 flex px-4 py-4">
                    <a
                        classes={LINK_SMALL_CSS}
                        href={tr!(about_url)}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { tr!(about) }
                    </a>
                    <Link<PublicRoute> classes={LINK_SMALL_CSS} to={PublicRoute::Help}>
                        { tr!(help_and_support) }
                    </Link<PublicRoute>>
                </div>
            </div>
        </form>
    }
}
