use crate::{css::*, hooks::use_layout_ctx, model::ConfirmationType, tr};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::{PublicRoute, model::SendConfirmationLink, services};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub confirmation_type: ConfirmationType,
}

#[function_component(Confirmation)]
pub fn confirmation(props: &Props) -> Html {
    let layout = use_layout_ctx();

    let signup_email = use_state(|| "".to_string());
    let email_sent = use_bool_toggle(false);

    let (header_label, email_sent_label, submit_label) = match props.confirmation_type {
        ConfirmationType::PasswordReset => (tr!(password_reset), tr!(reset_email_sent), tr!(reset)),
        ConfirmationType::Registration => (tr!(register), tr!(signup_email_sent), tr!(sign_up)),
    };

    {
        let layout = layout.clone();
        use_mount(move || {
            layout.set_pub_route_back_button_layout(header_label);
        });
    }

    let oninput_signup_email = {
        let signup_email = signup_email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            signup_email.set(input.value());
        })
    };

    let send_signup_email = {
        let signup_email = signup_email.clone();
        let email_sent = email_sent.clone();
        let confirmation_type = props.confirmation_type.clone();
        use_async(async move {
            email_sent.toggle();
            services::send_confirmation_link(SendConfirmationLink {
                email: (*signup_email).clone(),
                confirmation_type,
                server_address: services::requests::SERVER_ADDRESS.to_owned(),
            })
            .await
        })
    };

    let onsubmit = {
        let send_signup_email = send_signup_email.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            send_signup_email.run();
        })
    };

    html! {
        <form onsubmit={onsubmit}>
            <div class={BODY_DIV_CSS}>
                if *email_sent && send_signup_email.error.is_none() {
                    <div class="relative">
                        <label>{ email_sent_label }</label>
                    </div>
                } else {
                    <div class="relative">
                        <input
                            id="email"
                            type="email"
                            placeholder="Email"
                            class={INPUT_CSS}
                            value={(*signup_email).clone()}
                            oninput={oninput_signup_email}
                            required=true
                        />
                        <label for="email" class={INPUT_LABEL_CSS}>
                            <i class="icon-mail" />
                            { format!(" {}", tr!(email_address)) }
                        </label>
                    </div>
                    if props.confirmation_type == ConfirmationType::Registration {
                        <div class="relative flex justify-between sm:text-base">
                            <Link<PublicRoute> classes={LINK_CSS} to={PublicRoute::Login}>
                                { tr!(have_an_account) }
                            </Link<PublicRoute>>
                        </div>
                    }
                    <div
                        class="relative"
                    >
                        <button class={SUBMIT_BTN_CSS}>{ submit_label }</button>
                    </div>
                }
            </div>
        </form>
    }
}
