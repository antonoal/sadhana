use std::collections::HashMap;

use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_async, use_list, use_map, use_mount};
use yew_router::prelude::*;

use crate::{
    components::draggable_list::{DraggableList, Item},
    context::HeaderButton,
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    model::UserPractice,
    routes::AppRoute,
    services::{
        delete_user_practice, get_user_practices, reorder_user_practices, update_user_practice,
    },
    tr,
};

#[function_component(UserPractices)]
pub fn user_practices() -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let ordered_practices = use_list(vec![]);
    let local_practices = use_map(HashMap::default());

    let server_practices =
        use_cache_aware_async(get_user_practices().map(|res| res.user_practices));

    let reorder_practices = {
        let op = ordered_practices.clone();
        use_async(async move {
            let op = op.current().to_owned();
            reorder_user_practices(&op).await
        })
    };

    {
        // Load state on mount
        let server_practices = server_practices.clone();
        let layout = layout.clone();
        use_mount(move || {
            layout.set_app_service_extra_layout(
                true,
                Some(tr!(practices)),
                Some(AppRoute::Default),
                vec![HeaderButton::new_icon_redirect(
                    AppRoute::NewUserPractice,
                    "icon-plus",
                )],
            );
            server_practices.run();
        });
    }

    {
        let local = local_practices.clone();
        use_effect_with(server_practices.clone(), move |practices| {
            log::debug!("All Practices loaded. Initialising active practices");

            local.set(
                practices
                    .data
                    .iter()
                    .flat_map(|inner| inner.iter())
                    .map(|p| (p.id.clone(), p.to_owned()))
                    .collect(),
            );
            || ()
        });
    }

    let is_hidden = {
        let local = local_practices.clone();
        Callback::from(move |id: String| !local.current().get(&id).unwrap().is_active)
    };

    let toggle_hidden = {
        let local = local_practices.clone();
        Callback::from(move |id: String| {
            let updated = {
                let current = local.current().get(&id).unwrap().clone();

                UserPractice {
                    is_active: !current.is_active,
                    ..current
                }
            };

            {
                let local = local.clone();
                spawn_local(async move {
                    update_user_practice(&updated)
                        .await
                        .map(|_| local.update(&id, updated))
                        .unwrap()
                });
            }
        })
    };

    let delete = {
        let server_practices = server_practices.clone();
        Callback::from(move |id: String| {
            log::debug!("Deleting user practice {:?}", id);

            {
                let server_practices = server_practices.clone();
                spawn_local(async move {
                    delete_user_practice(&id)
                        .await
                        .map(|_| {
                            server_practices.run();
                        })
                        .unwrap()
                });
            }
        })
    };

    let rename = {
        let nav = nav.clone();
        Callback::from(move |(id, _): (String, String)| {
            nav.push(&AppRoute::EditUserPractice { id });
        })
    };

    let reorder = {
        let op = ordered_practices.clone();
        let rp = reorder_practices.clone();
        Callback::from(move |practices: Vec<Item>| {
            op.set(practices.iter().map(|i| i.id.clone()).collect());
            rp.run();
        })
    };

    fn display_practice(p: &UserPractice) -> String {
        format!(
            "{} {}",
            p.practice,
            p.is_required
                .and_then(|required| required.then_some("*"))
                .unwrap_or_default()
        )
    }

    html! {
        <div class={format!("mx-auto max-w-md {}", BODY_DIV_BASE_CSS)}>
            if !(server_practices.loading || local_practices.current().is_empty()) {
                <DraggableList
                    items={server_practices
                                .data
                                .as_ref()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|p|
                                    Item {
                                        id: p.id.clone(),
                                        name: display_practice(local_practices.current().get(&p.id).unwrap())
                                    }
                                )
                                .collect::<Vec<_>>()}
                    toggle_hidden={toggle_hidden.clone()}
                    is_hidden={is_hidden.clone()}
                    rename={rename.clone()}
                    rename_popup_label={tr!(enter_new_practice_name)}
                    request_new_name=false
                    delete={delete.clone()}
                    delete_popup_label={tr!(delete_practice_warning)}
                    reorder={reorder.clone()}
                />
            }
            <p
                class="text-xs text-zinc-500 dark:text-zinc-200"
            >
                { tr!(asterisk_is_required_memo) }
            </p>
        </div>
    }
}
