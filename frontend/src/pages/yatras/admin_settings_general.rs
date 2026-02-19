use tw_merge::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::*;

use crate::{
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    model::Yatra,
    routes::AppRoute,
    services::{get_yatra, update_yatra},
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(AdminSettingsGeneral)]
pub fn admin_settings_general(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let yatra_state = use_state(Yatra::default);

    let yatra = use_cache_aware_async(get_yatra(&props.yatra_id).map(|resp| resp.yatra));

    let save = {
        let yatra_id = props.yatra_id.clone();
        let yatra = (*yatra_state).clone();
        let nav = nav.clone();
        use_async(async move {
            update_yatra(yatra_id.as_str(), yatra)
                .await
                .map(|_| {
                    nav.push(&AppRoute::YatraAdminSettings {
                        id: yatra_id.to_string(),
                    })
                })
        })
    };

    {
        let yatra = yatra.clone();
        let layout = layout.clone();
        let yatra_id = props.yatra_id.to_string();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(yatra_general)),
                Some(AppRoute::YatraAdminSettings { id: yatra_id }),
                vec![],
            );
            yatra.run();
        });
    }

    {
        let yatra_state = yatra_state.clone();
        use_effect_with(yatra.clone(), move |y| {
            if let Some(y) = y.data.as_ref() {
                yatra_state.set(y.clone());
            }
            || ()
        });
    }

    let yatra_name_onchange = {
        let yatra_state = yatra_state.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_yatra = (*yatra_state).clone();
            new_yatra.name = input.value();
            yatra_state.set(new_yatra);
        })
    };

    let checkbox_onclick = {
        let yatra_state = yatra_state.clone();
        Callback::from(move |ev: MouseEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            let mut new_state = (*yatra_state).clone();
            new_state.show_stability_metrics = input.checked();
            yatra_state.set(new_state);
        })
    };

    html! {
        <form
            onsubmit={let save = save.clone();
                Callback::from(move |e: SubmitEvent| {
                    e.prevent_default();
                    save.run();
                })}
        >
            <div class={tw_merge!(BODY_DIV_BASE_CSS, "mx-auto max-w-md")}>
                <div class={BODY_DIV_CSS}>
                    <div class="relative">
                        <input
                            onchange={yatra_name_onchange}
                            type="text"
                            id="yatra_name"
                            value={yatra_state.name.clone()}
                            placeholder="yatra_name"
                            autocomplete="off"
                            required=true
                            class={tw_merge!(INPUT_CSS, "text-center")}
                        />
                        <label for="yatra_name" class={INPUT_LABEL_CSS}>
                            <i class="icon-doc" />
                            { tr!(yatra_name_label) }
                        </label>
                    </div>
                    <div>
                        <label class="flex justify-between whitespace-nowrap pl-2 pr-2">
                            <span>
                                <i class="icon-tick" />
                                { tr!(yatra_show_stability) }
                            </span>
                            <div class="flex">
                                <input
                                    type="checkbox"
                                    class={CHECKBOX_INPUT_CSS}
                                    onclick={checkbox_onclick}
                                    checked={yatra_state.show_stability_metrics}
                                />
                            </div>
                        </label>
                        <div class="pt-2">
                            <p class="text-xs text-zinc-500 dark:text-zinc-200">
                                { tr!(yatra_heatmap_memo_p1) }
                            </p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-200">
                                { tr!(yatra_heatmap_memo_p2) }
                            </p>
                            <p class="text-xs text-zinc-500 dark:text-zinc-200">
                                { tr!(yatra_heatmap_memo_p3) }
                            </p>
                        </div>
                    </div>
                    <div class="relative">
                        <button class={SUBMIT_BTN_CSS}>
                            <i class="icon-save" />
                            { tr!(save) }
                        </button>
                    </div>
                </div>
            </div>
        </form>
    }
}
