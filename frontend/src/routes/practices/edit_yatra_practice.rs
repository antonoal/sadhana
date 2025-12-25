use std::{fmt::Display, str::FromStr};

use tw_merge::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_router::prelude::use_navigator;

use crate::{
    components::{
        blank_page::{BlankPage, HeaderButtonProps},
        grid::Grid,
        list_errors::ListErrors,
        summary_details::SummaryDetails,
    },
    css::*,
    i18n::*,
    model::{PracticeDataType, PracticeEntryValue, YatraPractice},
    services::{get_yatra_practice, update_yatra_practice},
    AppRoute,
};

const COLOUR_ZONE_DATA_TYPES: [PracticeDataType; 3] = [
    PracticeDataType::Time,
    PracticeDataType::Duration,
    PracticeDataType::Int,
];

#[derive(Clone, Debug, PartialEq)]
pub struct ColourZonesConfig {
    pub better_direction: BetterDirection,
    pub zones: Vec<Band>,
    pub no_value_colour: ZoneColour,
}

impl Default for ColourZonesConfig {
    fn default() -> Self {
        Self {
            better_direction: BetterDirection::Higher,
            zones: vec![
                Band {
                    from: None,
                    to: None,
                    colour: ZoneColour::Red,
                },
                Band {
                    from: None,
                    to: None,
                    colour: ZoneColour::Yellow,
                },
                Band {
                    from: None,
                    to: None,
                    colour: ZoneColour::Green,
                },
            ],
            no_value_colour: ZoneColour::Neutral,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BetterDirection {
    Higher,
    Lower,
}

impl Display for BetterDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BetterDirection::Higher => write!(f, "Higher"),
            BetterDirection::Lower => write!(f, "Lower"),
        }
    }
}

impl FromStr for BetterDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Higher" => Ok(BetterDirection::Higher),
            "Lower" => Ok(BetterDirection::Lower),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Band {
    pub from: Option<PracticeEntryValue>,
    pub to: Option<PracticeEntryValue>,
    pub colour: ZoneColour,
}

impl Default for Band {
    fn default() -> Self {
        Self {
            from: None,
            to: None,
            colour: ZoneColour::Neutral,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ZoneColour {
    Neutral,
    Red,
    Yellow,
    Green,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
    pub practice_id: AttrValue,
}

#[function_component(EditYatraPractice)]
pub fn edit_yatra_practice(props: &Props) -> Html {
    let nav = use_navigator().unwrap();
    let practice = use_state(YatraPractice::default);
    let color_zones_hidden = use_bool_toggle(true);
    let color_zones_enabled = use_bool_toggle(false);
    let colour_zones_config = use_state(|| ColourZonesConfig::default());

    let current_practice = {
        let practice_id = props.practice_id.clone();
        let yatra_id = props.yatra_id.clone();
        use_async(async move {
            get_yatra_practice(&yatra_id, &practice_id)
                .await
                .map(|res| res.practice)
        })
    };

    let update_practice = {
        let practice = practice.clone();
        let nav = nav.clone();
        let yatra_id = props.yatra_id.clone();
        use_async(async move {
            update_yatra_practice(&yatra_id, &practice)
                .await
                .map(|_| nav.back())
        })
    };

    {
        let current_practice = current_practice.clone();
        use_mount(move || {
            current_practice.run();
        });
    }

    {
        let practice = practice.clone();
        let color_zones_enabled = color_zones_hidden.clone();
        use_effect_with(current_practice.clone(), move |current| {
            current.data.iter().for_each(|p| {
                color_zones_enabled.set(!COLOUR_ZONE_DATA_TYPES.contains(&p.data_type));
                practice.set(p.to_owned())
            });
            || ()
        });
    }

    let num_zones_onchange = {
        let colour_zones_config = colour_zones_config.clone();
        let color_zones_enabled = color_zones_enabled.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut config = (*colour_zones_config).clone();
            let num_zones = input.value().parse::<usize>().unwrap_or(0);
            if num_zones == 0 {
                color_zones_enabled.set(false);
                return;
            }
            color_zones_enabled.set(true);
            config.zones.resize(num_zones, Band::default());
            colour_zones_config.set(config);
        })
    };

    let better_when_onchange = {
        let colour_zones_config = colour_zones_config.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut config = (*colour_zones_config).clone();
            config.better_direction = input
                .value()
                .as_str()
                .parse()
                .unwrap_or(BetterDirection::Higher);
            colour_zones_config.set(config);
        })
    };

    let practice_oninput = {
        let practice = practice.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_practice = (*practice).clone();
            new_practice.practice = input.value();
            practice.set(new_practice);
        })
    };

    let onsubmit = {
        let update_user_practice = update_practice.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            update_user_practice.run();
        })
    };

    html! {
        <form {onsubmit}>
            <BlankPage
                left_button={HeaderButtonProps::back_to(AppRoute::YatraAdminSettings { id: props.yatra_id.to_string() })}
                loading={update_practice.loading}
                header_label={Locale::current().practice()}
                >
                <ListErrors error={current_practice.error.clone()} />
                <ListErrors error={update_practice.error.clone()} />
                <div class={BODY_DIV_CSS}>
                    <div class="relative">
                        <input
                            id="practice"
                            type="text"
                            placeholder="Practice"
                            class={INPUT_CSS}
                            value={practice.practice.clone()}
                            oninput={practice_oninput}
                            required=true
                            />
                        <label for="practice"
                            class={INPUT_LABEL_CSS}>
                            <i class="icon-doc"></i>{format!(" {}", Locale::current().name())}
                        </label>
                    </div>
                    {if !*color_zones_hidden {
                        html! {
                            <SummaryDetails label={"Colour zones"}>
                                <div class="relative">
                                    // <label class="text-lg">{"Colour zones"}</label>
                                    <div class="pt-2">
                                        <p class="text-xs text-zinc-500 dark:text-zinc-200">{"TODO: [i18n] Colour zones make the yatra table more visual by painting each value cell red, green, or yellow depending on the cell value."}</p>
                                    </div>
                                </div>
                                <div class={BODY_DIV_CSS}>
                                <div class="relative">
                                    <select
                                        onchange={num_zones_onchange}
                                        class={
                                            tw_merge!(
                                                "appearance-none",
                                                INPUT_CSS,
                                                "text-center [text-align-last:center] has-value")
                                        } >
                                        <option class={"text-black"} selected=true value={"0"}>{"Disabled"}</option>
                                        <option class={"text-black"} selected=false value={"3"}>{"3 (Red, Yellow, Green)"}</option>
                                        <option class={"text-black"} selected=false value={"2"}>{"2"}</option>
                                    </select>
                                    <label
                                        class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"></i>{" Number of zones:"}
                                    </label>
                                </div>
                                <div class="relative">
                                    <select
                                        disabled={!*color_zones_enabled}
                                        onchange={better_when_onchange}
                                        class={
                                            tw_merge!(
                                                "appearance-none",
                                                INPUT_CSS,
                                                "text-center [text-align-last:center] has-value")
                                        } >
                                        <option class={"text-black"} selected=true value={BetterDirection::Higher.to_string()}>{BetterDirection::Higher.to_string()}</option>
                                        <option class={"text-black"} selected=false value={BetterDirection::Lower.to_string()}>{BetterDirection::Lower.to_string()}</option>
                                    </select>
                                    <label
                                        class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"></i>{" Better when:"}
                                    </label>
                                </div>
                                // {for colour_zones_config.zones.iter().enumerate().map(|zone| {
                                //     html! {
                                //         {"42"}
                                //     }
                                // })}
                                <div class="relative">
                                    <input
                                        disabled={!*color_zones_enabled}
                                        autocomplete="off"
                                        // id={idx.to_string()}
                                        type="text"
                                        pattern="[0-9]*"
                                        // onblur={onblur_time.clone()}
                                        // onfocus={onfocus_time.clone()}
                                        // oninput={oninput_time.clone()}
                                        // onkeydown={onkeydown_time_dur.clone()}
                                        value={"05:00"} //{ value.iter().find_map(|v| v.as_time_str()).unwrap_or_default() }
                                        class={tw_merge!(INPUT_CSS, "text-center")}
                                        // placeholder={idx.to_string()}
                                        />
                                    <label /*for={idx.to_string()}*/ class={INPUT_LABEL_CSS}>
                                        <i class="icon-clock"></i>
                                        {"Amber starts at: "}
                                    </label>
                                </div>
                                <div class="relative">
                                    <input
                                        disabled={!*color_zones_enabled}
                                        autocomplete="off"
                                        // id={idx.to_string()}
                                        type="text"
                                        pattern="[0-9]*"
                                        // onblur={onblur_time.clone()}
                                        // onfocus={onfocus_time.clone()}
                                        // oninput={oninput_time.clone()}
                                        // onkeydown={onkeydown_time_dur.clone()}
                                        value={"06:00"} //{ value.iter().find_map(|v| v.as_time_str()).unwrap_or_default() }
                                        class={tw_merge!(INPUT_CSS, "text-center")}
                                        // placeholder={idx.to_string()}
                                        />
                                    <label /*for={idx.to_string()}*/ class={INPUT_LABEL_CSS}>
                                        <i class="icon-clock"></i>
                                        {"Red starts at: "}
                                    </label>
                                </div>
                                <div class="relative">
                                    <select
                                        disabled={!*color_zones_enabled}
                                        // onchange={onchange.clone()}
                                        // id={idx.to_string()}
                                        class={
                                            tw_merge!(
                                                "appearance-none",
                                                INPUT_CSS,
                                                "text-center [text-align-last:center]",
                                                if true {// TODO: value.as_ref().is_some_and(|v| !v.to_string().is_empty()) {
                                                    "has-value"
                                                } else {
                                                    ""
                                                })
                                        } >
                                        <option class={"text-black"} selected=true value={"Neutral"}>{"Neutral"}</option>
                                        <option class={"text-black"} selected=false value={"Red"}>{"Red"}</option>
                                        <option class={"text-black"} selected=false value={"Red"}>{"Yellow"}</option>
                                        <option class={"text-black"} selected=false value={"Red"}>{"Green"}</option>
                                    </select>
                                    <label
                                        // for={idx.to_string()}
                                        class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"></i>{" No value:"}
                                    </label>
                                </div>
                                <div class="relative">
                                    // <label>{"Preview:"}</label>
                                    <label
                                        // for={idx.to_string()}
                                        class={INPUT_SELECT_LABEL_CSS}>
                                        <i class="icon-rounds"></i>{" Preview:"}
                                    </label>
                                    <Grid
                                        // header={["🟢 Green", "🟡 Yellow", "🔴 Red"].iter().map(|s| s.to_string()).collect::<Vec<_>>()}
                                        header={["", "", ""].iter().map(|s| s.to_string()).collect::<Vec<_>>()}
                                        data={vec![["4:30", "5:30", "6:30"].iter().map(|s| Some(PracticeEntryValue::Text(s.to_string())) ).collect()]}
                                        // zones={TODO: }
                                        // TODO: no_header=true
                                    />
                                </div>
                                // <div class="relative">
                                //     <input
                                //         id="practice"
                                //         type="text"
                                //         placeholder="Practice"
                                //         class={INPUT_CSS}
                                //         // value={practice.practice.clone()}
                                //         // oninput={practice_oninput.clone()}
                                //         required=true
                                //         />
                                //     <label for="practice"
                                //         class={INPUT_LABEL_CSS}>
                                //         <i class="icon-doc"></i>{""}
                                //     </label>
                                // </div>
                                // <div class="relative">
                                //     <button type="submit" class={SUBMIT_BTN_CSS}>{Locale::current().save()}</button>
                                // </div>
                                </div>
                            </SummaryDetails>
                        }
                    } else {
                        html!{}
                    }}
                    <div class="relative">
                        <button type="submit" class={SUBMIT_BTN_CSS}>{Locale::current().save()}</button>
                    </div>
                </div>
            </BlankPage>
        </form>
    }
}
