use tw_merge::*;
use yew::prelude::*;
use yew_hooks::use_bool_toggle;

use crate::{
    components::{Calendar, ErrorsBanner, Footer, MonthCalendar},
    context::{HeaderButton as HeaderBtn, NetworkStatus},
    hooks::use_layout_ctx,
    layouts::{pane_header_buttons, pane_offline_banner},
    pages::Input,
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(TwoPane)]
pub fn two_pane(props: &Props) -> Html {
    let network_status = use_context::<NetworkStatus>().expect("NetworkStatus context not found");
    let layout = use_layout_ctx();
    let show_month_cal = use_bool_toggle(false);
    let show_ctx_menu = use_bool_toggle(false);
    let left_panel_disabled = !layout.show_footer;

    let month_cal_toggle = {
        let show_month_cal = show_month_cal.clone();
        Callback::from(move |_| {
            show_month_cal.toggle();
        })
    };

    let month_cal_button = layout
        .calendar
        .show
        .then(|| HeaderBtn::month_calendar(month_cal_toggle.clone()));

    let left_buttons = month_cal_button
        .into_iter()
        .chain(layout.left_buttons.clone().into_iter())
        .collect::<Vec<_>>();
    let all_header_buttons = left_buttons
        .into_iter()
        .chain(layout.right_buttons.clone().into_iter())
        .collect::<Vec<_>>();

    html! {
        <>
            if !network_status.online {
                { pane_offline_banner(tr!(offline_msg)) }
            }
            <div
                class={tw_merge!(
                    "h-screen w-full overflow-hidden",
                    "lg:grid lg:grid-cols-[minmax(360px,420px)_1fr]",
                    if !network_status.online { "pt-4" } else { "" }
                )}
            >
                <aside
                    class={tw_merge!(
                        "relative h-full overflow-y-auto border-r border-zinc-200/50 dark:border-zinc-700/50 px-4",
                        if left_panel_disabled {
                            "select-none opacity-60 pointer-events-none"
                        } else {
                            ""
                        }
                    )}
                    inert={left_panel_disabled.to_string()}
                    aria-disabled={left_panel_disabled.to_string()}
                >
                    <div class="mx-auto w-full max-w-[420px] pb-8">
                        <div class="mb-4 pt-5 text-center">
                            <img class="logo h-14 inline-block" src="/images/logo.png" />
                        </div>
                        <Input with_single_pane_layout=false />
                    </div>
                </aside>
                <section
                    class="relative h-full overflow-y-auto"
                >
                    <div
                        class={tw_merge!(
                            "mx-auto w-full max-w-7xl px-6 pt-5",
                            if layout.show_footer { "pb-24" } else { "pb-8" }
                        )}
                    >
                        <div class="mb-6 grid grid-cols-[1fr_auto_1fr] items-start gap-4">
                            <div class="flex min-h-10 items-center justify-start">
                                { pane_header_buttons(&all_header_buttons, show_ctx_menu.clone()) }
                            </div>
                            <div class="min-w-[20rem] justify-self-center">
                                if layout.calendar.show {
                                    <Calendar />
                                }
                            </div>
                            <div class="min-h-10" />
                        </div>
                        if let Some(title) = &layout.title {
                            <div class="pb-5 text-center">
                                <h5 class="text-xl font-medium text-zinc-500 dark:text-zinc-100">
                                    { title }
                                </h5>
                            </div>
                        }
                        if *show_month_cal {
                            <MonthCalendar close={month_cal_toggle.clone()} />
                        }
                        <ErrorsBanner />
                        { props.children.clone() }
                    </div>
                    if layout.show_footer {
                        <Footer show_home=false inside_parent=true />
                    }
                </section>
            </div>
        </>
    }
}
