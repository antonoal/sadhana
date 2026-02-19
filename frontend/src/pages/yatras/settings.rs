use std::collections::HashSet;

use common::error::AppError;
use gloo_dialogs::{confirm, prompt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_list, use_mount, use_set};
use yew_router::prelude::*;

use crate::{
    context::{CtxMenuEntry, HeaderButton},
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    model::{PracticeDataType, YatraUserPractice},
    routes::AppRoute,
    services::{
        create_yatra, get_user_practices, get_yatra, get_yatra_user_practices, is_yatra_admin,
        update_yatra_user_practices, yatra_leave,
    },
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(YatraSettings)]
pub fn yatra_settings(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let yatra = {
        let yatra_id = props.yatra_id.clone();
        use_cache_aware_async(get_yatra(&yatra_id).map(|resp| resp.yatra))
    };

    let yatra_user_practices = use_cache_aware_async(
        get_yatra_user_practices(props.yatra_id.as_str()).map(|res| res.practices),
    );

    let user_practices = {
        use_cache_aware_async(get_user_practices().map(|res| {
            res.user_practices
                .into_iter()
                .filter(|p| p.is_active)
                .collect::<Vec<_>>()
        }))
    };
    let nav = use_navigator().unwrap();

    let leave = {
        let yatra_id = props.yatra_id.clone();
        let nav = nav.clone();
        use_async(async move {
            yatra_leave(&yatra_id)
                .await
                .map(|_| nav.push(&AppRoute::Yatras))
        })
    };

    let mapped_practices = use_list(vec![]);
    let mapped_user_practices = use_set(HashSet::<String>::default());

    let is_admin = {
        let yatra_id = props.yatra_id.clone();
        use_cache_aware_async(is_yatra_admin(yatra_id.as_str()).map(|resp| resp.is_admin))
    };

    let save = {
        let mapped_practices = mapped_practices.clone();
        let yatra_id = props.yatra_id.clone();
        let nav = nav.clone();
        use_async(async move {
            let mapped_practices = mapped_practices.current().to_owned();
            update_yatra_user_practices(yatra_id.as_str(), &mapped_practices)
                .await
                .map(|_| nav.push(&AppRoute::Yatras))
        })
    };

    let new_yatra = use_async(async move {
        if let Some(yatra_name) =
            prompt(&tr!(yatra_new_name_prompt), None).filter(|s| !s.trim().is_empty())
        {
            create_yatra(yatra_name.trim().to_owned())
                .await
                .map(|res| res.yatra)
        } else {
            Err(AppError::UnprocessableEntity(vec![]))
        }
    });

    {
        let is_admin = is_admin.clone();
        let layout = layout.clone();
        let yatra = yatra.clone();
        let yatra_practices = yatra_user_practices.clone();
        let user_practices = user_practices.clone();
        use_mount(move || {
            layout.set_app_service_layout(false, None, Some(AppRoute::Yatras), vec![]);
            is_admin.run();
            yatra_practices.run();
            user_practices.run();
            yatra.run();
        });
    }

    {
        let nav = nav.clone();
        use_effect_with(new_yatra.clone(), move |res| {
            log::debug!("Created yatra {:?}", res.data);
            res.data
                .iter()
                .for_each(|y| nav.push(&AppRoute::YatraSettings { id: y.id.clone() }));
            || ()
        });
    }

    {
        let mapped_practices = mapped_practices.clone();
        let mapped_user_practices = mapped_user_practices.clone();
        use_effect_with(yatra_user_practices.clone(), move |yp| {
            yp.data
                .iter()
                .for_each(|inner| mapped_practices.set(inner.clone()));
            mapped_user_practices.set(
                yp.data
                    .iter()
                    .flat_map(|inner| inner.iter())
                    .filter_map(|yp| yp.user_practice.clone())
                    .collect::<HashSet<_>>(),
            );

            || ()
        })
    }

    let leave_onclick = {
        let leave = leave.clone();
        Callback::from(move |_: MouseEvent| {
            if confirm(&tr!(yatra_leave_warning)) {
                leave.run();
            }
        })
    };

    let practice_onchange = {
        let mapped = mapped_practices.clone();
        let mapped_user_practices = mapped_user_practices.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();

            let yatra_practice = input.id();
            let user_practice = input.value();

            if !user_practice.is_empty() {
                log::debug!("Inserting into mapped_user_practices {}", user_practice);
                mapped_user_practices.insert(user_practice.clone());
            }

            let (idx, value) = {
                let mapped_current = mapped.current();
                let (idx, value) = mapped_current
                    .iter()
                    .enumerate()
                    .find(|(_, v)| v.yatra_practice.practice == yatra_practice)
                    .unwrap();

                value.user_practice.iter().for_each(|up| {
                    log::debug!("Removing from mapped_user_practices {}", up);
                    mapped_user_practices.remove(up);
                });

                (
                    idx,
                    YatraUserPractice {
                        yatra_practice: value.yatra_practice.clone(),
                        user_practice: Some(user_practice).filter(|v| !v.is_empty()),
                    },
                )
            };

            log::debug!("Updating mapped_practices with {:?}", value);
            mapped.update(idx, value);
        })
    };

    let onsubmit = {
        let save = save.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            save.run();
        })
    };

    fn practice_icon(data_type: &PracticeDataType) -> String {
        (match data_type {
            PracticeDataType::Int => "icon-rounds",
            PracticeDataType::Bool => "icon-tick",
            PracticeDataType::Time => "icon-clock",
            PracticeDataType::Text => "icon-doc",
            PracticeDataType::Duration => "icon-clock",
        })
        .into()
    }

    let practices = {
        mapped_practices
            .current()
            .iter()
            .map(|yp| {
                html! {
                    <div class="relative">
                        <select
                            class={INPUT_CSS}
                            id={yp.yatra_practice.practice.clone()}
                            onchange={practice_onchange.clone()}
                        >
                            <option class="text-black" selected={yp.user_practice.is_none()}>
                                { "" }
                            </option>
                            { user_practices
                                    .data
                                    .iter()
                                    .flat_map(|inner| inner.iter())
                                    .filter(|up| {
                                        up.data_type == yp.yatra_practice.data_type
                                            && (!mapped_user_practices.current().contains(&up.practice)
                                                || yp
                                                    .user_practice
                                                    .iter()
                                                    .any(|p| *p == up.practice))
                                    })
                                    .map(|up| {
                                        html! {
                                            <option
                                                class={ "text-black" }
                                                selected={ yp.user_practice.iter().any(|p| **p == up.practice) }
                                                value={ up.practice.clone() } >
                                                { up.practice.clone() }
                                            </option>
                                        }
                                    })
                                    .collect::<Html>() }
                        </select>
                        <label
                            for={{ yp.yatra_practice.practice.clone() }}
                            class={INPUT_LABEL_CSS}
                        >
                            <i class={practice_icon(&yp.yatra_practice.data_type)} />
                            { format!(" {}: ", yp.yatra_practice.practice) }
                        </label>
                    </div>
                }
            })
            .collect::<Html>()
    };

    let create_yatra_onclick = {
        let create = new_yatra.clone();
        Callback::from(move |_: MouseEvent| {
            create.run();
        })
    };

    let admin_settings_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YatraAdminSettings {
                id: yatra_id.to_string(),
            });
        })
    };

    {
        let layout = layout.clone();
        let yatra = yatra.clone();
        let is_admin = is_admin.clone();
        let leave_onclick = leave_onclick.clone();
        let create_yatra_onclick = create_yatra_onclick.clone();
        let admin_settings_onclick = admin_settings_onclick.clone();
        use_effect_with((yatra, is_admin), move |(y, is_admin)| {
            let mut actions = vec![
                CtxMenuEntry::action(leave_onclick.clone(), "icon-logout", &tr!(yatra_leave)),
                CtxMenuEntry::action(
                    create_yatra_onclick.clone(),
                    "icon-plus",
                    &tr!(yatra_create),
                ),
            ];
            if is_admin.data.unwrap_or(false) {
                actions.push(CtxMenuEntry::action(
                    admin_settings_onclick.clone(),
                    "icon-settings",
                    &tr!(yatra_modify_admin),
                ));
            }
            layout.set_app_service_layout(
                false,
                y.data.iter().map(|v| v.name.clone()).next(),
                Some(AppRoute::Yatras),
                vec![HeaderButton::ctx_menu("icon-ellipsis-vertical", actions)],
            );
            || ()
        });
    }

    html! {
        <form {onsubmit}>
            <div class={BODY_DIV_NO_PADDING_CSS}>
                <div class="pt-2">
                    <p class="text-xs text-zinc-500 dark:text-zinc-200">
                        { tr!(yatra_mapping_info) }
                    </p>
                </div>
                { practices }
                <div class="relative">
                    <button class={SUBMIT_BTN_CSS}>
                        <i class="icon-tick" />
                        { format!(" {}", tr!(save)) }
                    </button>
                </div>
            </div>
        </form>
    }
}
