use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::{
    css::*,
    hooks::{use_layout_ctx, use_user_ctx},
    model::*,
    services, tr,
};

#[function_component(EditUser)]
pub fn edit_user() -> Html {
    let layout = use_layout_ctx();
    let user_info = use_state(UpdateUser::default);
    let editing = use_bool_toggle(false);
    let user_ctx = use_user_ctx();
    let form_ref = use_node_ref();

    let update_user = {
        let user_info = user_info.clone();
        use_async(async move { services::update_user((*user_info).clone()).await })
    };

    let edit_onclick = {
        let editing = editing.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            editing.toggle();
        })
    };

    {
        let layout = layout.clone();
        let form_ref = form_ref.clone();
        let edit_onclick = edit_onclick.clone();
        use_effect_with(*editing, move |editing| {
            layout.set_app_service_edit_layout(
                *editing,
                tr!(user_details),
                form_ref.clone(),
                edit_onclick.clone(),
            );
            || ()
        });
    }

    {
        let user_info = user_info.clone();
        use_effect_with(user_ctx.clone(), move |ctx| {
            user_info.set(UpdateUser::new(&ctx.name));
            || ()
        });
    }

    let name_oninput = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_info = (*user_info).clone();
            new_info.name = input.value();
            user_info.set(new_info);
        })
    };

    let name_onblur = {
        let user_info = user_info.clone();
        Callback::from(move |_: FocusEvent| {
            let mut new_info = (*user_info).clone();
            new_info.name = user_info.name.trim().to_string();
            user_info.set(new_info);
        })
    };

    let onreset = {
        let editing = editing.clone();
        let user_info = user_info.clone();
        let ctx = user_ctx.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            user_info.set(UpdateUser::new(&ctx.name));
            editing.toggle();
        })
    };

    let onsubmit = {
        let update_user = update_user.clone();
        let editing = editing.clone();
        let user_info = user_info.clone();
        let ctx = user_ctx.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !user_info.name.is_empty() && ctx.name != user_info.name {
                update_user.run();
            }
            editing.toggle();
        })
    };

    html! {
        <form ref={form_ref} {onsubmit} {onreset}>
            <div class={BODY_DIV_CSS}>
                <div class="relative">
                    <input
                        id="email"
                        type="email"
                        placeholder="Email"
                        class={INPUT_CSS}
                        value={user_ctx.email.clone()}
                        disabled=true
                        required=true
                    />
                    <label for="email" class={INPUT_LABEL_CSS}>
                        <i class="icon-mail" />
                        { format!(" {}", tr!(email_address)) }
                    </label>
                </div>
                <div class="relative">
                    <input
                        id="name"
                        type="text"
                        placeholder="Name"
                        class={INPUT_CSS}
                        value={user_info.name.clone()}
                        oninput={name_oninput}
                        onblur={name_onblur}
                        readonly={!*editing}
                        minlength="3"
                        pattern="[\\S\\s]+[\\S]+"
                    />
                    <label for="name" class={INPUT_LABEL_CSS}>
                        <i class="icon-user" />
                        { format!(" {}", tr!(name)) }
                    </label>
                </div>
            </div>
        </form>
    }
}
