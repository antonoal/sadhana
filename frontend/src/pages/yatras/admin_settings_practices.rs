use tw_merge::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_hooks::{use_async, use_list, use_mount};
use yew_router::prelude::*;

use crate::{
    components::{DraggableItem, DraggableList},
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    routes::AppRoute,
    services::{delete_yatra_practice, get_yatra_practices, reorder_yatra_practices},
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(AdminSettingsPractices)]
pub fn admin_settings_practices(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let ordered_practices = use_list(vec![]);

    let all_practices = use_cache_aware_async(
        get_yatra_practices(props.yatra_id.as_str()).map(|res| res.practices),
    );

    let reorder_practices = {
        let yatra_id = props.yatra_id.clone();
        let ordered_practices = ordered_practices.current().to_owned();
        use_async(
            async move { reorder_yatra_practices(yatra_id.as_str(), ordered_practices).await },
        )
    };

    {
        let all_practices = all_practices.clone();
        let layout = layout.clone();
        let yatra_id = props.yatra_id.to_string();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(yatra_practices)),
                Some(AppRoute::YatraAdminSettings { id: yatra_id }),
                vec![],
            );
            all_practices.run();
        });
    }

    let edit_practice = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |(id, _): (String, String)| {
            nav.push(&AppRoute::EditYatraPractice {
                id: yatra_id.clone(),
                practice_id: id,
            });
        })
    };

    let reorder = {
        let ordered_practices = ordered_practices.clone();
        let reorder_practices = reorder_practices.clone();
        Callback::from(move |practices: Vec<DraggableItem>| {
            ordered_practices.set(practices.into_iter().map(|p| p.id).collect());
            reorder_practices.run();
        })
    };

    let delete = {
        let all_practices = all_practices.clone();
        let yatra_id = props.yatra_id.clone();
        Callback::from(move |practice_id: String| {
            let all_practices = all_practices.clone();
            let yatra_id = yatra_id.clone();
            spawn_local(async move {
                delete_yatra_practice(yatra_id.as_str(), &practice_id)
                    .await
                    .map(|_| all_practices.run())
                    .unwrap();
            });
        })
    };

    let new_yatra_practice_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::NewYatraPractice {
                id: yatra_id.clone(),
            });
        })
    };

    html! {
        <div class={tw_merge!(BODY_DIV_BASE_CSS, "mx-auto max-w-md")}>
            if !all_practices.loading {
                <DraggableList
                    items={all_practices.data
                        .as_ref()
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|p| DraggableItem { id: p.id.clone(), name: p.practice.clone() })
                        .collect::<Vec<_>>()}
                    toggle_hidden_enabled=false
                    toggle_hidden={Callback::from(|_| {})}
                    is_hidden={Callback::from(|_| false)}
                    rename={edit_practice}
                    request_new_name=false
                    rename_popup_label={tr!(enter_new_practice_name)}
                    delete={delete}
                    delete_popup_label={tr!(yatra_delete_practice_warning)}
                    reorder={reorder}
                />
                <div>
                    <button class={BTN_CSS} onclick={new_yatra_practice_onclick}>
                        <i class="icon-plus" />
                        { tr!(add_new_practice) }
                    </button>
                </div>
            }
        </div>
    }
}
