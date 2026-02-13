use std::collections::HashMap;

use anyhow::{Context, Result, anyhow};
use chrono::NaiveDate;
use common::error::AppError;
use csv::{Reader, ReaderBuilder, StringRecord};
use futures::future::join_all;
use gloo::file::File;
use gloo_dialogs::confirm;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FileList, HtmlInputElement};
use yew::prelude::*;
use yew_hooks::{
    UseAsyncHandle, UseListHandle, use_bool_toggle, use_effect_update_with_deps, use_list,
    use_mount,
};
use yew_router::prelude::{Link, use_navigator};

use crate::{
    components::DATE_FORMAT,
    css::*,
    hooks::{use_async_with_error, use_cache_aware_async, use_layout_ctx},
    i18n::*,
    model::{DiaryDay, DiaryEntry, PracticeDataType, Value},
    routes::AppRoute,
    services::{get_user_practices, save_diary_owned},
    tr,
};

#[function_component(Import)]
pub fn import() -> Html {
    let layout = use_layout_ctx();
    let csv_data = use_state(|| None::<String>);
    let headers = use_list(vec![]);
    let saving = use_bool_toggle(false);
    let successes: UseListHandle<(NaiveDate, DiaryDay)> = use_list(vec![]);
    let failures = use_list(vec![]);
    let nav = use_navigator().unwrap();

    let all_practices = use_cache_aware_async(get_user_practices().map(|res| {
        res.user_practices
            .iter()
            .map(|up| (up.practice.clone(), up.data_type))
            .collect::<HashMap<_, _>>()
    }));

    let save: UseAsyncHandle<Vec<()>, AppError> = {
        let successes = successes.clone();
        let nav = nav.clone();
        use_async_with_error(async move {
            let successes = successes.current().to_owned();

            log::debug!("Saving: {:?}", successes);

            let res = join_all(
                successes
                    .iter()
                    .map(|(cob, dd)| save_diary_owned(cob, dd.clone())),
            )
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>();

            if let Ok(res) = res.as_ref() {
                confirm(&tr!(import_success_msg, SuccessQty(&res.len().to_string())));
                nav.push(&AppRoute::Settings);
            }

            res
        })
    };

    {
        let layout = layout.clone();
        let all = all_practices.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(import_csv)),
                Some(AppRoute::Settings),
                vec![],
            );
            all.run();
        });
    }

    fn scv_reader(data: &str) -> Reader<&[u8]> {
        ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(true)
            .from_reader(data.as_bytes())
    }

    {
        let headers = headers.clone();
        let all_practices = all_practices.clone();
        use_effect_update_with_deps(
            move |data| {
                if let Some(data) = data.as_ref() {
                    let mut rdr = scv_reader(data);
                    let hs = rdr
                        .headers()
                        .unwrap()
                        .iter()
                        .map(|h| h.to_owned())
                        .collect::<Vec<_>>();

                    if let Some(practices) = all_practices.data.as_ref() {
                        headers.set(
                            hs.into_iter()
                                .map(|h| (h.clone(), practices.get(&h).copied()))
                                .collect(),
                        );
                    }
                }
                || ()
            },
            csv_data.clone(),
        );
    }

    let upload_files = {
        let scv_data = csv_data.clone();
        move |files: Option<FileList>| {
            let mut result = Vec::new();

            if let Some(files) = files {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                result.extend(files);
            }

            if let Some(f) = result.into_iter().next() {
                let csv_data = scv_data.clone();
                spawn_local(async move {
                    gloo::file::futures::read_as_text(&f)
                        .await
                        .map(|data| {
                            log::debug!("Read file: {:?}", data);
                            csv_data.set(Some(data));
                        })
                        .unwrap()
                });
            }
        }
    };

    let upload_onclick = {
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            upload_files(input.files())
        })
    };

    fn to_diary_day(
        row: StringRecord,
        headers: &[(String, Option<PracticeDataType>)],
    ) -> Result<(NaiveDate, DiaryDay)> {
        let mut diary_day = vec![];
        let mut it = headers.iter().zip(row.iter());

        let (_, cob) = it
            .next()
            .ok_or_else(|| anyhow!(tr!(import_row_parse_err)))?;

        let cob_date = NaiveDate::parse_from_str(cob, DATE_FORMAT)
            .with_context(|| tr!(import_cob_parse_err, Cob(cob)))?;

        for (h, data_type, v) in it.filter_map(|((h, data_type), v)| data_type.map(|dt| (h, dt, v)))
        {
            let value = (!v.trim().is_empty())
                .then(|| Value::try_from((&data_type, v)))
                .transpose()?;
            let entry = DiaryEntry {
                practice: h.to_owned(),
                data_type,
                dropdown_variants: None,
                value,
            };
            diary_day.push(entry);
        }

        Ok((cob_date, DiaryDay { diary_day }))
    }

    let onsubmit = {
        let data = csv_data.clone();
        let headers = headers.clone();
        let saving = saving.clone();
        let successes = successes.clone();
        let failures = failures.clone();
        let save = save.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            saving.toggle();
            if let Some(data) = data.as_ref() {
                let mut rdr = scv_reader(data);
                let headers = headers.current();

                for (row_num, row) in rdr.records().enumerate() {
                    let dd = row
                        .map_err(anyhow::Error::from)
                        .with_context(|| tr!(import_row_parse_err))
                        .and_then(|row| to_diary_day(row, &headers));
                    if let Ok(dd) = dd {
                        successes.push(dd);
                    } else {
                        failures.push((
                            Some(row_num),
                            dd.err().map(|e| e.to_string()).unwrap_or_default(),
                        ));
                    }
                }

                if failures.current().is_empty() {
                    save.run();
                }
            }

            saving.toggle();
        })
    };

    let columns_view = {
        log::debug!(
            "Building columns list from headers: {:?}",
            headers.current()
        );
        html! {
            <>
                <div>
                    <h5 class="text-center mb-4 text-xl font-medium leading-tight">
                        { tr!(import_discovered_columns) }
                    </h5>
                    <p class="text-zinc-500 dark:text-zinc-200">
                        { tr!(import_discovered_columns_memo) }
                    </p>
                </div>
                { for headers.current().iter().skip(1).filter(|(_, dt)| dt.is_some()).map(|(h, dt)| html! {
                    <div class="relative">
                        <select class={INPUT_CSS} id={(*h).to_owned() }>
                            <option class={"text-black"} disabled=true selected={dt == &Some(PracticeDataType::Int)}>{tr!(integer)}</option>
                            <option class={"text-black"} disabled=true selected={dt == &Some(PracticeDataType::Time)}>{tr!(time)}</option>
                            <option class={"text-black"} disabled=true selected={dt == &Some(PracticeDataType::Bool)}>{tr!(boolean)}</option>
                            <option class={"text-black"} disabled=true selected={dt == &Some(PracticeDataType::Text)}>{tr!(text)}</option>
                            <option class={"text-black"} disabled=true selected={dt == &Some(PracticeDataType::Duration)}>{tr!(duration)}</option>
                        </select>
                        <label for={(*h).to_owned()} class={INPUT_LABEL_CSS}>{h}</label>
                    </div>
                }) }
                <div>
                    <h5 class="text-center mb-4 text-xl font-medium leading-tight">
                        { tr!(import_unmatched_columns) }
                    </h5>
                    <p class="text-zinc-500 dark:text-zinc-200">
                        { tr!(import_unmatched_columns_memo) }
                    </p>
                </div>
                <div class="space-y-0">
                    { for headers.current().iter().skip(1).filter(|(_, dt)| dt.is_none()).map(|(h, _)| html! {
                        <div
                            class="flex w-full justify-center align-baseline"
                            id={(*h).to_owned()}
                            >
                            <label class="flex w-full justify-between whitespace-nowrap mb-6">
                                <span>{(*h).to_owned()}</span>
                            </label>
                            <label>
                                <Link<AppRoute>
                                    to={AppRoute::NewUserPracticeWithName { practice: (*h).to_owned() }}
                                >
                                    <i class="icon-plus" />
                                </Link<AppRoute>>
                            </label>
                        </div>
                    }) }
                </div>
                <div class="relative">
                    <button type="submit" class={SUBMIT_BTN_CSS}>{ tr!(import_csv) }</button>
                </div>
            </>
        }
    };

    let file_picker = html! {
        <div class="relative">
            <input
                id="file-upload"
                type="file"
                accept="text/csv"
                onchange={upload_onclick}
                multiple=false
                class={format!("{} text-center", INPUT_CSS)}
            />
            <label for="file-upload" class={INPUT_LABEL_CSS}>
                <i class="icon-doc" />
                { format!(" {}: ", tr!(import_file_select)) }
            </label>
        </div>
    };

    let list_failures =
        {
            failures
            .current()
            .iter()
            .map(|(line, msg)| {
                let line = line.iter().map(|l| (l + 1).to_string()).next().unwrap_or_default();
                html! {
                    <span class="text-zinc-500 dark:text-zinc-200">
                        { format!(
                            "{}{msg}",
                            if !line.is_empty() {
                                format!("{}: ", tr!(import_failure_line_num_msg, LineNum(&line)))
                            } else {
                                Default::default()
                            }
                        ) }
                    </span>
                }
            })
            .collect::<Html>()
        };

    html! {
        <form {onsubmit}>
            <div class={BODY_DIV_CSS}>
                <div>
                    <h5 class="text-center mb-4 text-xl font-medium leading-tight">
                        { tr!(import_instructions_header) }
                    </h5>
                    { for tr!(import_instructions_body)
                        .lines()
                        .map(|l| html! {<p class="text-zinc-500 dark:text-zinc-200">{l}</p>}) }
                </div>
                { if csv_data.is_none() {
                    file_picker
                } else if !failures.current().is_empty() {
                    list_failures
                } else {
                    columns_view
                } }
            </div>
        </form>
    }
}
