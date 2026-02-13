use urlencoding::decode;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_bool_toggle, use_mount};
use yew_router::prelude::use_navigator;

use crate::{
    AppRoute,
    css::*,
    hooks::use_layout_ctx,
    model::{NewUserPractice, NewYatraPractice},
    pages::{
        DROPDOWN_PRACTICE_TYPES,
        practices::{COLOUR_ZONE_DATA_TYPES, Mode},
    },
    services::{create_user_practice, create_yatra_practice},
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mode: Mode,
    #[prop_or_default]
    pub practice: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct FormData {
    practice: String,
    data_type: String,
    is_required: Option<bool>,
    dropdown_variants: Option<String>,
}

#[function_component(NewPractice)]
pub fn new_practice(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let form_data = use_state(|| FormData {
        practice: props
            .practice
            .iter()
            .flat_map(|p| decode(p).map(|s| s.into_owned()).ok())
            .next()
            .unwrap_or_default(),
        ..FormData::default()
    });
    let is_dropdown = use_bool_toggle(false);
    let nav = use_navigator().unwrap();

    let save = {
        let form = form_data.clone();
        let nav = nav.clone();
        let target = props.mode.clone();
        let is_dropdown = is_dropdown.clone();
        use_async(async move {
            (match target {
                Mode::UserPractice => {
                    let variants = form.dropdown_variants.clone().filter(|_| *is_dropdown);
                    let new_practice = NewUserPractice {
                        practice: form.practice.trim().to_owned(),
                        data_type: form.data_type.as_str().try_into().unwrap(),
                        is_active: true,
                        is_required: form.is_required,
                        dropdown_variants: variants,
                    };
                    create_user_practice(new_practice).await
                }
                Mode::YatraPractice { yatra_id } => {
                    create_yatra_practice(
                        &yatra_id,
                        NewYatraPractice {
                            yatra_id: yatra_id.clone(),
                            practice: form.practice.clone(),
                            data_type: form.data_type.as_str().try_into().unwrap(),
                        },
                    )
                    .await
                }
            })
            .map(|_| nav.back())
        })
    };

    let onsubmit = {
        let save = save.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            save.run();
        })
    };

    let practice_oninput = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();

            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form = (*form_data).clone();
            form.practice = input.value().to_owned();
            form_data.set(form);
        })
    };

    let dropdown_variants_oninput = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            // TODO: typecheck the entered variants
            e.prevent_default();

            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form = (*form_data).clone();
            let variants = input.value().to_owned();
            if !variants.trim().is_empty() {
                form.dropdown_variants = Some(variants);
            } else {
                form.dropdown_variants = None;
            }
            form_data.set(form);
        })
    };

    let data_type_onchange = {
        let form_data = form_data.clone();
        let is_dropdown = is_dropdown.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();

            let input: HtmlInputElement = e.target_unchecked_into();

            if !input.value().is_empty() {
                input.set_custom_validity("");
            }

            let mut form = (*form_data).clone();
            form.data_type = input.value();
            form_data.set(form);
            is_dropdown.set(false);
        })
    };

    {
        let layout = layout.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                true,
                Some(tr!(add_new_practice)),
                Some(AppRoute::UserPractices),
                vec![],
            );
            if let Some(element) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.get_element_by_id("data_type"))
            {
                let html: HtmlInputElement = element.unchecked_into();
                html.set_custom_validity(&tr!(data_type_cant_be_empty));
            }
        });
    }

    let is_dropdown_onclick = {
        let is_dropdown = is_dropdown.clone();
        Callback::from(move |e: MouseEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            is_dropdown.set(input.checked());
        })
    };

    let required_onclick = {
        let form = form_data.clone();
        Callback::from(move |e: MouseEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_form = (*form).clone();
            new_form.is_required = Some(input.checked());
            form.set(new_form);
        })
    };

    html! {
        <form {onsubmit}>
            <div class={BODY_DIV_CSS}>
                <div class="relative">
                    <input
                        autocomplete="off"
                        id="practice_name"
                        type="text"
                        oninput={practice_oninput.clone()}
                        class={INPUT_CSS}
                        placeholder="practice_name"
                        pattern="[^\\s]+?.*"
                        maxlength="64"
                        required=true
                        disabled={props.practice.is_some()}
                        value={form_data.practice.clone()}
                    />
                    <label for="practice_name" class={INPUT_LABEL_CSS}>
                        <i class="fa" />
                        { format!(" {}: ", tr!(practice_name)) }
                    </label>
                </div>
                <div class="relative">
                    <select
                        class={INPUT_CSS}
                        id="data_type"
                        onchange={data_type_onchange}
                        required=true
                    >
                        <option class="text-black" value="int">{ tr!(integer) }</option>
                        <option class="text-black" value="time">{ tr!(time) }</option>
                        <option class="text-black" value="bool">{ tr!(boolean) }</option>
                        <option class="text-black" value="text">{ tr!(text) }</option>
                        <option class="text-black" value="duration">
                            { tr!(duration_in_mins) }
                        </option>
                        <option
                            class="text-black"
                            value=""
                            selected=true
                            disabled=true
                            style="display: none"
                        >
                            { tr!(select_data_type) }
                        </option>
                    </select>
                    <label for="data_type" class={INPUT_LABEL_CSS}>
                        <i class="fa" />
                        { format!(" {}: ", tr!(data_type)) }
                    </label>
                </div>
                if matches!(&props.mode, Mode::YatraPractice { yatra_id: _ })
                    && COLOUR_ZONE_DATA_TYPES
                        .map(|dt| dt.to_string())
                        .contains(&form_data.data_type) {
                    <p class="text-xs text-zinc-500 dark:text-zinc-200">
                        { tr!(colour_zones_new_practice_how_to_memo) }
                    </p>
                }
                if *is_dropdown {
                    <div class="relative">
                        <input
                            class={INPUT_CSS}
                            autocomplete="off"
                            id="dropdown_variants"
                            type="text"
                            placeholder="dropdown variants"
                            maxlength="1024"
                            oninput={dropdown_variants_oninput.clone()}
                            value={form_data.dropdown_variants.clone()}
                        />
                        <label for="dropdown_variants" class={INPUT_LABEL_CSS}>
                            <i class="fa" />
                            { format!(" {}: ", tr!(dropdown_variants)) }
                        </label>
                    </div>
                }
                if props.mode == Mode::UserPractice {
                    if DROPDOWN_PRACTICE_TYPES.contains(&form_data.data_type.as_str()) {
                        <div class="relative">
                            <label class="flex justify-between whitespace-nowrap pl-2 pr-2">
                                <span>
                                    <i class="icon-tick" />
                                    { format!(" {}: ", tr!(is_dropdown)) }
                                </span>
                                <div>
                                    <input
                                        id="is_dropdown"
                                        type="checkbox"
                                        class={CHECKBOX_INPUT_CSS}
                                        checked={*is_dropdown}
                                        onclick={is_dropdown_onclick.clone()}
                                    />
                                </div>
                            </label>
                        </div>
                    }
                    <div
                        class="relative"
                    >
                        <label class="flex justify-between whitespace-nowrap pl-2 pr-2">
                            <span>
                                <i class="icon-tick" />
                                { format!(" {}: ", tr!(is_required)) }
                            </span>
                            <div>
                                <input
                                    id="mandatory"
                                    type="checkbox"
                                    class={CHECKBOX_INPUT_CSS}
                                    onclick={required_onclick.clone()}
                                />
                            </div>
                        </label>
                        <div class="pt-2">
                            <p class="text-xs text-zinc-500 dark:text-zinc-200">
                                { tr!(is_required_memo) }
                            </p>
                        </div>
                    </div>
                }
                <div
                    class="relative"
                >
                    <button type="submit" class={SUBMIT_BTN_CSS}>{ tr!(save) }</button>
                </div>
            </div>
        </form>
    }
}
