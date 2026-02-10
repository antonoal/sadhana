use crate::{
    components::pwd::Pwd,
    css::*,
    hooks::use_layout_ctx,
    model,
    routes::PublicRoute,
    services::{get_signup_link_details, reset_pwd},
    tr,
};
use gloo_dialogs::alert;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_router::prelude::Redirect;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(PwdReset)]
pub fn pwd_reset(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let pwd = use_state(String::default);

    let pwd_onchange = {
        let pwd = pwd.clone();
        Callback::from(move |new_pwd: String| {
            pwd.set(new_pwd);
        })
    };

    let email = {
        let confirmation_id = props.id.clone();
        use_async(async move { get_signup_link_details(confirmation_id.as_str()).await })
    };

    {
        let email = email.clone();
        let layout = layout.clone();
        use_mount(move || {
            layout.set_pub_route_back_button_layout(tr!(password_reset));
            email.run();
        });
    }

    let finished = use_bool_toggle(false);

    let reset_pwd = {
        let email = email.clone();
        let pwd = pwd.clone();
        let finished = finished.clone();
        use_async(async move {
            reset_pwd(model::ResetPassword {
                confirmation_id: email.data.as_ref().unwrap().confirmation.id.clone(),
                password: (*pwd).clone(),
            })
            .await
            .map(|_| {
                finished.toggle();
            })
        })
    };

    let onsubmit = {
        let reset_pwd = reset_pwd.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            reset_pwd.run();
        })
    };

    if *finished {
        alert(&tr!(reset_success_alert));
        return html! { <Redirect<PublicRoute> to={PublicRoute::Login} /> };
    }

    html! {
        if email.error.is_none() {
            <form {onsubmit}>
                <div class={BODY_DIV_CSS}>
                    <Pwd onchange={pwd_onchange} />
                    <div class="relative">
                        <button class={SUBMIT_BTN_CSS}>{ tr!(save) }</button>
                    </div>
                </div>
            </form>
        }
    }
}
