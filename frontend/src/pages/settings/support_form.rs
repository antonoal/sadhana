use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;

use crate::{
    css::*,
    hooks::{use_async_with_error, use_layout_ctx},
    routes::AppRoute,
    services::send_support_message,
    tr,
};

#[function_component(SupportForm)]
pub fn support_form() -> Html {
    let layout = use_layout_ctx();
    let subject = use_state(String::default);
    let message = use_state(String::default);

    let submit = {
        let subject = subject.clone();
        let message = message.clone();
        use_async_with_error(async move { send_support_message(&subject, &message).await })
    };

    {
        let layout = layout.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(sf_send_us_message)),
                Some(AppRoute::Help),
                vec![],
            );
        });
    }

    let onsubmit = {
        let submit = submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submit.run();
        })
    };

    let oninput_subject = {
        let subject = subject.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            subject.set(input.value());
        })
    };

    let oninput_message = {
        let message = message.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            message.set(input.value());
        })
    };

    html! {
        <form {onsubmit}>
            <div class={BODY_DIV_CSS}>
                <div class="relative">
                    <input
                        autocomplete="off"
                        id="subject"
                        type="text"
                        oninput={oninput_subject}
                        class={INPUT_CSS}
                        placeholder="subject"
                        maxlength="128"
                        required=true
                    />
                    <label for="subject" class={INPUT_LABEL_CSS}>
                        <i class="fa" />
                        { format!(" {}: ", tr!(sf_subject)) }
                    </label>
                </div>
                <div class="relative">
                    <textarea
                        class={TEXTAREA_CSS}
                        maxlength="4000"
                        rows="12"
                        required=true
                        placeholder="message"
                        oninput={oninput_message}
                    />
                    <label for="message" class={INPUT_LABEL_CSS}>
                        <i class="icon-doc" />
                        { format!(" {}: ", tr!(sf_message)) }
                    </label>
                </div>
                <div class="relative">
                    <button class={SUBMIT_BTN_CSS}>
                        <i class="icon-send" />
                        { format!(" {}", tr!(sf_send)) }
                    </button>
                </div>
            </div>
        </form>
    }
}
