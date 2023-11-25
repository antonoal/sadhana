use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{components::summary_details::SummaryDetails, css::*, model::UserPractice};

use super::GridReport;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub all_practices: Vec<UserPractice>,
    pub report: GridReport,
    pub report_name: AttrValue,
    pub report_onchange: Callback<(String, GridReport)>,
}

#[function_component(GridEditor)]
pub fn grid_editor(props: &Props) -> Html {
    let report = use_state(|| props.report.clone());
    let report_name = use_state(|| props.report_name.to_string());

    {
        let cb = props.report_onchange.clone();
        use_effect_with_deps(
            move |(name, rep)| {
                cb.emit(((**name).clone(), (**rep).clone()));
                || ()
            },
            (report_name.clone(), report.clone()),
        );
    }

    let checkbox_onclick = {
        let report = report.clone();
        Callback::from(move |ev: MouseEvent| {
            let input: HtmlInputElement = ev.target_unchecked_into();
            let practice = input.id();
            let mut new_practices = report.practices.clone();
            if input.checked() {
                new_practices.insert(practice);
            } else {
                new_practices.remove(&practice);
            }
            report.set(GridReport::new(new_practices));
        })
    };

    let report_name_oninput = {
        let report_name = report_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            report_name.set(input.value());
        })
    };

    let practices = props.all_practices.iter().map(|p| {
        html! {
            <div class="relative" key={p.id.clone()} >
                <label class="flex justify-between whitespace-nowrap pl-2 pr-2">
                    <span class=""><i class="icon-tick"></i>{format!(" {}: ", p.practice)}</span>
                    <input
                        type="checkbox"
                        onclick={checkbox_onclick.clone()}
                        id={p.id.clone()}
                        checked={report.practices.contains(&p.id)}
                        />
                </label>
            </div>
        }
    });

    html! {
        <div class="pt-8 text-zinc-500 dark:text-zinc-100">
            <SummaryDetails tab_index={1} label={"Settings"}>//FIXME: localise
                <div class="pt-8">
                    <div class={TWO_COLS_CSS}>
                        <div class="relative">
                            <input
                                type="text"
                                id="name"
                                placeholder="Name"
                                value={(*report_name).clone()}
                                oninput={report_name_oninput}
                                class={INPUT_CSS}
                                required = true
                                autocomplete="off"
                                />
                            <label for="name" class={INPUT_LABEL_CSS}>
                                { format!(" {}", "Chart Name")} //FIXME: localise
                            </label>
                        </div>
                    </div>
                </div>
            </SummaryDetails>
            <SummaryDetails tab_index={1} label={"Practices"}>//FIXME: localise
                <div class="pt-8">
                    <div class={TWO_COLS_CSS}>
                        {for practices}
                    </div>
                </div>
            </SummaryDetails>
            <div class="relative">
                <button class={BTN_CSS} /*onclick={add_trace_onclick.clone()}*/>{"Delete Chart"}</button> //FIXME: localise
            </div>
        </div>
    }
}
