use super::{SelectedReportId, SELECTED_REPORT_ID_KEY};
use crate::{
    context::HeaderButton,
    css::*,
    hooks::{use_async_with_error, use_layout_ctx},
    pages::charts::ReportForm,
    services::report::create_new_report,
    tr,
};
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::use_navigator;

enum ReportType {
    Graph,
    Grid,
}

#[function_component(CreateReport)]
pub fn create_report() -> Html {
    let layout = use_layout_ctx();
    let report_name = use_state(String::default);
    let report_type = use_state(|| ReportType::Graph);
    let form_ref = use_node_ref();
    let nav = use_navigator().unwrap();

    let create = {
        let report_name = report_name.clone();
        let report_type = report_type.clone();
        let nav = nav.clone();
        use_async_with_error(async move {
            let report = match *report_type {
                ReportType::Graph => ReportForm::default_graph(&*report_name),
                ReportType::Grid => ReportForm::default_grid(&*report_name),
            };
            create_new_report(report)
                .await
                .map(|res| {
                    LocalStorage::set(SELECTED_REPORT_ID_KEY, SelectedReportId::new(res.report_id))
                        .unwrap()
                })
                .map(|_| nav.back())
        })
    };

    {
        let layout = layout.clone();
        let form_ref = form_ref.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                false,
                Some(tr!(report_add_new)),
                None,
                vec![HeaderButton::submit(form_ref)],
            );
        });
    }

    let report_type_onchange = {
        let report_type = report_type.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();

            let input: HtmlInputElement = e.target_unchecked_into();

            let new_type = match input.value().as_str() {
                "graph" => ReportType::Graph,
                "grid" => ReportType::Grid,
                _ => unreachable!(),
            };

            report_type.set(new_type);
        })
    };

    let report_name_oninput = {
        let report_name = report_name.clone();
        Callback::from(move |e: InputEvent| {
            e.prevent_default();
            let input: HtmlInputElement = e.target_unchecked_into();
            report_name.set(input.value());
        })
    };

    let onsubmit = {
        let create = create.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            create.run();
        })
    };

    html! {
        <form ref={form_ref} {onsubmit}>
            <div class={BODY_DIV_CSS}>
                <div class="relative">
                    <input
                        type="text"
                        placeholder="Name"
                        value={(*report_name).clone()}
                        id="report_name"
                        oninput={report_name_oninput}
                        class={INPUT_CSS}
                        required=true
                        autocomplete="off"
                    />
                    <label for="report_name" class={INPUT_LABEL_CSS}>
                        { tr!(report_name) }
                    </label>
                </div>
                <div class="relative">
                    <select class={INPUT_CSS} id="report_type" onchange={report_type_onchange}>
                        <option class="text-black" value="graph" selected=true>
                            { tr!(report_type_graph) }
                        </option>
                        <option class="text-black" value="grid">
                            { tr!(report_type_grid) }
                        </option>
                    </select>
                    <label for="report_type" class={INPUT_LABEL_CSS}>
                        { format!(" {}: ", tr!(report_type)) }
                    </label>
                </div>
            </div>
        </form>
    }
}
