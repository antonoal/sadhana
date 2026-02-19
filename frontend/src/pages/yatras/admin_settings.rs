use gloo_dialogs::confirm;
use inflector::Inflector;
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

use crate::{
    components::{ShareLink, can_share, emit_signal_callback, set_signal_callback},
    css::*,
    hooks::use_layout_ctx,
    routes::AppRoute,
    services::delete_yatra,
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(AdminSettings)]
pub fn admin_settings(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let share_signal = use_state(|| None::<Callback<_>>);
    let can_share = can_share();

    let delete_yatra = {
        let yatra_id = props.yatra_id.clone();
        let nav = nav.clone();
        use_async(async move {
            delete_yatra(yatra_id.as_str())
                .await
                .map(|_| nav.push(&AppRoute::Yatras))
        })
    };

    {
        let layout = layout.clone();
        let yatra_id = props.yatra_id.to_string();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(yatra_modify_admin)),
                Some(AppRoute::YatraSettings { id: yatra_id }),
                vec![],
            );
        });
    }

    let general_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YatraAdminSettingsGeneral {
                id: yatra_id.clone(),
            });
        })
    };

    let practices_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YatraAdminSettingsPractices {
                id: yatra_id.clone(),
            });
        })
    };

    let members_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YatraAdminSettingsMembers {
                id: yatra_id.clone(),
            });
        })
    };

    let stats_onclick = {
        let nav = nav.clone();
        let yatra_id = props.yatra_id.to_string();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::YatraAdminSettingsStats {
                id: yatra_id.clone(),
            });
        })
    };

    let delete_yatra_onclick = {
        let delete_yatra = delete_yatra.clone();
        Callback::from(move |_: MouseEvent| {
            if confirm(&tr!(yatra_delete_warning)) {
                delete_yatra.run();
            }
        })
    };

    fn menu_li(icon: &str, label: String, show_chevron: bool) -> Html {
        html! {
            <li>
                <div class={LI_DIV_CSS}>
                    <label>
                        <i class={format!("{icon} flex-shrink-0 w-5")} />
                        { label }
                    </label>
                    if show_chevron {
                        <i class="icon-chevron-right" />
                    }
                </div>
            </li>
        }
    }

    html! {
        <div class={format!("space-y-4 pt-14 mx-auto max-w-md {}", BODY_DIV_BASE_CSS)}>
            <ul onclick={general_onclick} class={UL_CSS}>
                { menu_li("icon-doc", tr!(yatra_general).to_sentence_case(), true) }
            </ul>
            <ul onclick={practices_onclick} class={UL_CSS}>
                { menu_li("icon-rounds", tr!(yatra_practices).to_sentence_case(), true) }
            </ul>
            <ul onclick={members_onclick} class={UL_CSS}>
                { menu_li("icon-user-group", tr!(yatra_members).to_sentence_case(), true) }
            </ul>
            <ul onclick={stats_onclick} class={UL_CSS}>
                { menu_li("icon-graph", tr!(yatra_stats_section_label).to_sentence_case(), true) }
            </ul>
            <ul onclick={emit_signal_callback(&share_signal)} class={UL_CSS}>
                { menu_li(if can_share {"icon-share"} else {"icon-doc-dup"}, tr!(yatra_share_link_menu), false) }
            </ul>
            <ShareLink
                relative_link={format!("/yatra/{}/join", props.yatra_id.as_str())}
                run_signal={set_signal_callback(&share_signal)}
            />
            <ul onclick={delete_yatra_onclick} class={UL_CSS}>
                { menu_li("icon-bin", tr!(yatra_delete).to_sentence_case(), false) }
            </ul>
        </div>
    }
}
