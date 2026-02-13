use crate::{
    components::pwd::Pwd,
    css::*,
    hooks::{use_async_with_error, use_layout_ctx},
    routes::AppRoute,
    services, tr,
};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_bool_toggle, use_mount};
use yew_router::prelude::use_navigator;

#[function_component(EditPassword)]
pub fn edit_password() -> Html {
    let layout = use_layout_ctx();
    let new_password = use_state(String::new);
    let current_pwd = use_state(String::default);
    let nav = use_navigator().unwrap();
    let show_pwd = use_bool_toggle(false);

    let update_password = {
        let new_password = new_password.clone();
        let current_pwd = current_pwd.clone();
        let nav = nav.clone();
        use_async_with_error(async move {
            services::update_user_password(&current_pwd, &new_password)
                .await
                .map(|_| nav.push(&AppRoute::Settings))
        })
    };

    {
        let layout = layout.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(change_password)),
                Some(AppRoute::Settings),
                vec![],
            );
        });
    }

    let pwd_onchange = {
        let new_password = new_password.clone();
        Callback::from(move |new_pwd: String| {
            new_password.set(new_pwd);
        })
    };

    let onsubmit = {
        let update_password = update_password.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            update_password.run();
        })
    };

    let oninput = {
        let pwd = current_pwd.clone();
        Callback::from(move |e: InputEvent| {
            let target: HtmlInputElement = e.target_unchecked_into();
            pwd.set(target.value());
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
                        id="current_pwd"
                        type={if *show_pwd {"text"} else {"password"}}
                        placeholder="Current Password"
                        class={INPUT_CSS}
                        value={(*current_pwd).clone()}
                        {oninput}
                        required=true
                        autocomplete="off"
                        minlength="5"
                        maxlength="256"
                    />
                    <div
                        class="absolute inset-y-0 right-0 pr-3 flex items-center text-sm leading-5"
                    >
                        <i
                            class={if *show_pwd {"icon-eye-cross"} else {"icon-eye"}}
                            onclick={toggle_show_pwd_onclick}
                        />
                    </div>
                    <label for="current_pwd" class={INPUT_LABEL_CSS}>
                        <i class="icon-key" />
                        { format!(" {}", tr!(current_password)) }
                    </label>
                </div>
                <Pwd onchange={pwd_onchange.clone()} required=true />
                <div class="relative">
                    <button class={SUBMIT_BTN_CSS}>
                        <i class="icon-login" />
                        { format!(" {}", tr!(save)) }
                    </button>
                </div>
            </div>
        </form>
    }
}
