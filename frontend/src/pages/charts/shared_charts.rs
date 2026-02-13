use super::base::ChartsBase;
use crate::{
    context::{CalendarState, LayoutAction, LayoutState, Session},
    hooks::{use_cache_aware_async, use_layout_ctx},
    pages::charts::{Report, SelectedReportId},
    services::{
        get_shared_practices,
        report::{get_shared_report_data, get_shared_reports},
        user_info,
    },
};
use common::ReportDuration;
use yew::prelude::*;
use yew_hooks::use_mount;

#[derive(Properties, Clone, PartialEq)]
pub struct SharedChartsProps {
    pub share_id: AttrValue,
}

#[function_component(SharedCharts)]
pub fn shared_charts(props: &SharedChartsProps) -> Html {
    let layout = use_layout_ctx();
    let session_ctx = use_context::<Session>().expect("No session state found");
    let active_report = use_state(|| None::<Report>);
    let duration = use_state(|| ReportDuration::Month);

    let user_info = {
        let share_id = props.share_id.clone();
        use_cache_aware_async(user_info(&share_id).map(|inner| inner.user))
    };

    let reports = use_cache_aware_async(get_shared_reports(&props.share_id).map(|res| res.reports));

    let practices = use_cache_aware_async(get_shared_practices(&props.share_id).map(|res| {
        res.user_practices
            .into_iter()
            .filter(|p| p.is_active)
            .collect::<Vec<_>>()
    }));

    let report_data = use_cache_aware_async(
        get_shared_report_data(&props.share_id, &session_ctx.selected_date, &duration)
            .map(|res| res.values),
    );

    {
        // Load state on mount
        let layout = layout.clone();
        let reports = reports.clone();
        let practices = practices.clone();
        let user_info = user_info.clone();
        let report_data = report_data.clone();
        use_mount(move || {
            layout.dispatch(LayoutAction::SetLayout(
                LayoutState::builder()
                    .show_footer(false)
                    .calendar(CalendarState {
                        highlight_incomplete: true,
                        ..Default::default()
                    })
                    .build(),
            ));
            reports.run();
            practices.run();
            user_info.run();
            report_data.run();
        });
    }

    {
        let report_data = report_data.clone();
        use_effect_with(session_ctx.clone(), move |_| {
            report_data.run();
            || ()
        });
    }

    {
        let layout = layout.clone();
        let user_info = user_info.clone();
        use_effect_with(user_info, move |u| {
            layout.dispatch(LayoutAction::SetLayout(
                LayoutState::builder()
                    .show_footer(false)
                    .calendar(CalendarState::default())
                    .title_opt(u.data.as_ref().map(|v| v.name.clone()))
                    .build(),
            ));
            || ()
        });
    }

    {
        let active = active_report.clone();
        use_effect_with(reports.clone(), move |reports| {
            active.set(
                reports
                    .data
                    .as_ref()
                    .and_then(|inner| inner.iter().next().cloned()),
            );
            || ()
        });
    }

    let dates_onchange = {
        let report_data = report_data.clone();
        let duration = duration.clone();
        Callback::from(move |dur| {
            duration.set(dur);
            report_data.run();
        })
    };

    let report_onchange = {
        let active = active_report.clone();
        let reports = reports.clone();
        Callback::from(move |id: SelectedReportId| {
            if let Some(reports) = reports.data.as_ref() {
                active.set(reports.iter().find(|r| r.id == id.report_id).cloned());
            }
        })
    };

    html! {
        if let Some(report) = active_report.as_ref() {
            if reports.data.is_some() {
                <ChartsBase
                    reports={reports.data.clone().unwrap_or_default()}
                    practices={practices.data.clone().unwrap_or_default()}
                    report_data={report_data.data.clone().unwrap_or_default()}
                    report={(*report).clone()}
                    {report_onchange}
                    {dates_onchange}
                />
            }
        }
    }
}
