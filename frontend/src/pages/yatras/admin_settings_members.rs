use tw_merge::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::{
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    routes::AppRoute,
    services::{delete_yatra_user, get_yatra_users, toggle_is_admin_yatra_user},
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(AdminSettingsMembers)]
pub fn admin_settings_members(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let action_user_id = use_mut_ref(|| None::<String>);

    let members =
        use_cache_aware_async(get_yatra_users(props.yatra_id.as_str()).map(|res| res.users));

    let delete_member = {
        let yatra_id = props.yatra_id.to_owned();
        let user_id = action_user_id.clone();
        use_async(async move {
            let user_id = user_id.borrow().to_owned();
            if let Some(user_id) = user_id {
                delete_yatra_user(yatra_id.as_str(), &user_id).await
            } else {
                Ok(())
            }
        })
    };

    let toggle_is_admin = {
        let yatra_id = props.yatra_id.to_owned();
        let user_id = action_user_id.clone();
        use_async(async move {
            let user_id = user_id.borrow().to_owned();
            if let Some(user_id) = user_id {
                toggle_is_admin_yatra_user(yatra_id.as_str(), &user_id).await
            } else {
                Ok(())
            }
        })
    };

    {
        let members = members.clone();
        let layout = layout.clone();
        let yatra_id = props.yatra_id.to_string();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(yatra_members)),
                Some(AppRoute::YatraAdminSettings { id: yatra_id }),
                vec![],
            );
            members.run();
        });
    }

    {
        let members = members.clone();
        use_effect_with(
            (toggle_is_admin.clone(), delete_member.clone()),
            move |_| {
                members.run();
                || ()
            },
        );
    }

    let delete_member_onclick = {
        let user_id = action_user_id.clone();
        let delete_member = delete_member.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let input: HtmlElement = e.target_unchecked_into();
            user_id.replace(Some(input.id()));
            delete_member.run();
        })
    };

    let toggle_is_admin_onclick = {
        let user_id = action_user_id.clone();
        let toggle_is_admin = toggle_is_admin.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let input: HtmlElement = e.target_unchecked_into();
            user_id.replace(Some(input.id()));
            toggle_is_admin.run();
        })
    };

    html! {
        <div class={tw_merge!(BODY_DIV_BASE_CSS, "mx-auto max-w-md")}>
            { for members
                .data
                .as_ref()
                .unwrap_or(&vec![])
                .iter()
                .enumerate()
                .map(|(idx, user)| html! {
                    <div class="flex w-full justify-center align-baseline">
                        <label class="flex w-full justify-between whitespace-nowrap mb-6" id={idx.to_string()}>
                            <span>{user.user_name.clone()}</span>
                        </label>
                        <div
                            class={tw_merge!(
                                "cursor-pointer text-sm mx-1 pt-1",
                                if user.is_admin { "text-amber-500" } else { "" }
                            )}
                            onclick={toggle_is_admin_onclick.clone()}
                            id={user.user_id.clone()}
                        >
                            {tr!(yatra_admin_label)}
                        </div>
                        <label>
                            <i onclick={delete_member_onclick.clone()} id={user.user_id.clone()} class="cursor-pointer icon-bin"/>
                        </label>
                    </div>
                }) }
        </div>
    }
}
