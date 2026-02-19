use strum::IntoEnumIterator;
use tw_merge::*;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{use_async, use_mount};
use yew_router::prelude::use_navigator;

use crate::{
    components::SummaryDetails,
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx},
    model::{Aggregation, PracticeDataType, TimeRange, Yatra, YatraStatistic, YatraStatistics},
    routes::AppRoute,
    services::{get_yatra, get_yatra_practices, update_yatra},
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(AdminSettingsStats)]
pub fn admin_settings_stats(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let yatra_state = use_state(Yatra::default);
    let stats_config = use_state(YatraStatistics::default);

    let yatra = use_cache_aware_async(get_yatra(&props.yatra_id).map(|resp| resp.yatra));
    let all_practices =
        use_cache_aware_async(get_yatra_practices(props.yatra_id.as_str()).map(|res| res.practices));

    let save = {
        let yatra_id = props.yatra_id.clone();
        let nav = nav.clone();
        let mut yatra = (*yatra_state).clone();
        yatra.statistics = (!stats_config.statistics.is_empty()).then_some((*stats_config).clone());
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
        let all_practices = all_practices.clone();
        let layout = layout.clone();
        let yatra_id = props.yatra_id.to_string();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(yatra_stats_section_label)),
                Some(AppRoute::YatraAdminSettings { id: yatra_id }),
                vec![],
            );
            yatra.run();
            all_practices.run();
        });
    }

    {
        let yatra_state = yatra_state.clone();
        let stats_config = stats_config.clone();
        use_effect_with(yatra.clone(), move |y| {
            if let Some(y) = y.data.as_ref() {
                yatra_state.set(y.clone());
                if let Some(conf) = &y.statistics {
                    stats_config.set(conf.clone());
                }
            }
            || ()
        });
    }

    let is_good_for_practice = |agg: &Aggregation, practice_id: &str| {
        all_practices
            .data
            .as_ref()
            .and_then(|practices| {
                practices
                    .iter()
                    .find(|p| p.id == practice_id)
                    .map(|p| p.data_type.to_owned())
            })
            .map(|dt| match dt {
                PracticeDataType::Int | PracticeDataType::Duration => true,
                PracticeDataType::Time => *agg != Aggregation::Sum,
                _ => *agg == Aggregation::Count,
            })
            .unwrap_or(*agg == Aggregation::Count)
    };

    let update_stats_state = |f: fn(&mut YatraStatistic, String)| {
        let stats_config = stats_config.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            let idx: usize = input
                .id()
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse index from id {}", input.id()));
            let mut next = (*stats_config).clone();
            if let Some(stat) = next.statistics.get_mut(idx) {
                f(stat, value);
            }
            stats_config.set(next);
        })
    };

    let stats_visibility_onchange = {
        let stats_config = stats_config.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut next = (*stats_config).clone();
            next.visible_to_all = input.value() == "everyone";
            stats_config.set(next);
        })
    };

    let stats_label_onchange = update_stats_state(|stat, v| stat.label = v);

    let stats_practice_onchange = update_stats_state(|stat, v| {
        stat.practice_id = v;
        stat.aggregation = Default::default();
    });

    let aggregation_onchange = update_stats_state(|stat, v| {
        if let Ok(agg) = v.parse() {
            stat.aggregation = agg;
        }
    });

    let stats_time_range_onchange = update_stats_state(|stat, v| {
        if let Ok(tr) = v.parse() {
            stat.time_range = tr;
        }
    });

    let stats_delete_onclick = {
        let stats_config = stats_config.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let input: HtmlElement = e.target_unchecked_into();
            let idx: usize = input
                .id()
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse index from id {}", input.id()));
            let mut next = (*stats_config).clone();
            if idx < next.statistics.len() {
                next.statistics.remove(idx);
            }
            stats_config.set(next);
        })
    };

    let add_stat_onclick = {
        let stats_config = stats_config.clone();
        Callback::from(move |_: MouseEvent| {
            let mut next = (*stats_config).clone();
            next.statistics.push(Default::default());
            stats_config.set(next);
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
                <div class={tw_merge!(if stats_config.statistics.is_empty() { BODY_DIV_CSS } else { BODY_DIV_BASE_CSS }, "pt-8")}>
                    <div class="relative">
                        <select
                            onchange={stats_visibility_onchange}
                            id="visibility"
                            class={tw_merge!(
                                INPUT_CSS,
                                "appearance-none text-center [text-align-last:center] has-value",
                            )}
                        >
                            <option value="admins" selected={!stats_config.visible_to_all}>
                                { tr!(yatra_stats_visibility_admins) }
                            </option>
                            <option value="everyone" selected={stats_config.visible_to_all}>
                                { tr!(yatra_stats_visibility_everyone) }
                            </option>
                        </select>
                        <label for="visibility" class={INPUT_SELECT_LABEL_CSS}>
                            <i class="icon-rounds" />
                            { tr!(yatra_stats_visible_to) }
                        </label>
                    </div>
                    { for stats_config.statistics.iter().enumerate().map(|(idx, stat)| html! {
                        <SummaryDetails
                            label={
                                if stat.label.is_empty() {
                                    tr!(yatra_stats_stat_heading, crate::i18n::Index(&(idx + 1).to_string()))
                                } else {
                                    stat.label.clone()
                                }
                            }
                            open={stat.label.is_empty() || stat.practice_id.is_empty()}
                        >
                            <div id={idx.to_string()} class={BODY_DIV_CSS}>
                                <div class="relative">
                                    <input
                                        onchange={stats_label_onchange.clone()}
                                        type="text"
                                        id={idx.to_string()}
                                        value={stat.label.to_owned()}
                                        placeholder="label"
                                        autocomplete="off"
                                        required=true
                                        class={tw_merge!(INPUT_CSS, "text-center")}
                                    />
                                    <label for={idx.to_string()} class={INPUT_LABEL_CSS}>
                                        <i class="icon-rounds"/>
                                        { tr!(yatra_stats_stat_label) }
                                    </label>
                                </div>
                                <div class="relative">
                                    <select
                                        onchange={stats_practice_onchange.clone()}
                                        id={idx.to_string()}
                                        required=true
                                        class={tw_merge!(
                                            INPUT_CSS,
                                            "appearance-none text-center [text-align-last:center]",
                                            if !stat.practice_id.is_empty() { "has-value" } else { "" }
                                        )}
                                    >
                                        <option class="text-black" value="" disabled=true style="display: none" selected={stat.practice_id.is_empty()} />
                                        { for all_practices.data.iter().flat_map(|inner| inner.iter()).map(|p| html! {
                                            <option value={p.id.to_owned()} selected={p.id == stat.practice_id}>{p.practice.as_str()}</option>
                                        }) }
                                    </select>
                                    <label for={idx.to_string()} class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"/>
                                        { tr!(yatra_stats_practice_label) }
                                    </label>
                                </div>
                                <div class="relative">
                                    <select
                                        onchange={aggregation_onchange.clone()}
                                        id={idx.to_string()}
                                        required=true
                                        class={tw_merge!(
                                            INPUT_CSS,
                                            "appearance-none text-center [text-align-last:center] has-value",
                                        )}
                                    >
                                        { for Aggregation::iter().filter(|agg| is_good_for_practice(agg, &stat.practice_id)).map(|agg| html! {
                                            <option value={agg.to_string()} selected={agg == stat.aggregation}>{agg.to_localised_string()}</option>
                                        }) }
                                    </select>
                                    <label for={idx.to_string()} class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"/>
                                        { tr!(yatra_stats_agg_label) }
                                    </label>
                                </div>
                                <div class="relative">
                                    <select
                                        onchange={stats_time_range_onchange.clone()}
                                        id={idx.to_string()}
                                        required=true
                                        class={tw_merge!(
                                            INPUT_CSS,
                                            "appearance-none text-center [text-align-last:center] has-value",
                                        )}
                                    >
                                        { for TimeRange::iter().map(|time_range| html! {
                                            <option value={time_range.to_string()} selected={time_range == stat.time_range}>{time_range.to_localised_string()}</option>
                                        }) }
                                    </select>
                                    <label for={idx.to_string()} class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"/>
                                        { tr!(yatra_stats_time_range_label) }
                                    </label>
                                </div>
                                <div>
                                    <button type="button" id={idx.to_string()} class={BTN_CSS} onclick={stats_delete_onclick.clone()}>
                                        <i class="icon-bin" />
                                        { tr!(yatra_stats_delete_stat) }
                                    </button>
                                </div>
                            </div>
                        </SummaryDetails>
                    }) }
                    <div>
                        <button type="button" class={BTN_CSS} onclick={add_stat_onclick}>
                            <i class="icon-plus" />
                            { tr!(yatra_stats_add_stat) }
                        </button>
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
