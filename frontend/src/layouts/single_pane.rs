use tw_merge::*;
use yew::prelude::*;
use yew_hooks::{UseToggleHandle, use_bool_toggle};

use crate::{
    components::{Calendar, ErrorsBanner, Footer, HeaderButton, MonthCalendar},
    context::{HeaderButton as HeaderBtn, NetworkStatus},
    hooks::use_layout_ctx,
    tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SinglePane)]
pub fn single_pane(props: &Props) -> Html {
    let network_status = use_context::<NetworkStatus>().expect("NetworkStatus context not found");
    let layout = use_layout_ctx();

    let show_month_cal = use_bool_toggle(false);
    let show_ctx_menu = use_bool_toggle(false);

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

    html! {
        <>
            if !network_status.online {
                <div
                    class="absolute bg-red-500 w-full h-4 top-[env(safe-area-inset-top)] z-10 overscroll-none"
                >
                    <p class="text-white text-center overflow-hidden text-xs">
                        { tr!(offline_msg) }
                    </p>
                </div>
            }
            <div
                id="content"
                class={tw_merge!(
                        "fixed pt-safe-top top-0 {} left-0 right-0 overflow-y-auto",
                        if layout.show_footer {"bottom-16"} else {"bottom-0"},
                        if !network_status.online {"top-4"} else {""}
                        )}
            >
                // 100vh-4rem means screen minus bottom-16; env(...) - the height of iPhone notch
                <div
                    class="bg-transparent min-h-[calc(100vh-4rem-env(safe-area-inset-top))] justify-center items-center py-[calc(0.5rem-env(safe-area-inset-top))] sm:py-[calc(3rem-env(safe-area-inset-top))]"
                >
                    <div class="w-full text-center relative">
                        <div class="absolute flex w-full h-full flex-col justify-center px-4">
                            <div class="relative">
                                <div
                                    class="relative sm:max-w-md md:max-w-md lg:max-w-lg xl:max-w-lg 2xl:max-w-lg mx-auto"
                                >
                                    <div class="relative flex justify-between py-10">
                                        { header_buttons(&left_buttons, show_ctx_menu.clone()) }
                                        { header_buttons(&layout.right_buttons, show_ctx_menu.clone()) }
                                    </div>
                                </div>
                            </div>
                        </div>
                        <img class="logo h-20 inline-block" src="/images/logo.png" />
                    </div>
                    <div
                        class="relative sm:max-w-xl md:max-w-3xl lg:max-w-4xl xl:max-w-5xl 2xl:max-w-7xl mx-auto"
                    >
                        <div
                            class="relative px-4 py-4 rounded-3xl sm:px-20 md:px-20 lg:px-20 xl:px-30 2xl:px-30"
                        >
                            if let Some(title) = &layout.title {
                                <div class="pb-5 text-center">
                                    <h5
                                        class="text-xl font-medium text-zinc-500 dark:text-zinc-100"
                                    >
                                        { title }
                                    </h5>
                                    // subtitle
                                    // { for props.header_sub_label.iter().map(|sl| {
                                    //     html!{<span class="text-sm text-zinc-300 dark:text-zinc-200">{sl}</span>}
                                    // }) }
                                </div>
                            }
                            if *show_month_cal {
                                <MonthCalendar close={month_cal_toggle.clone()} />
                            }
                            if layout.calendar.show {
                                <Calendar />
                            }
                            <ErrorsBanner/>
                            { props.children.clone() }
                        </div>
                    </div>
                </div>
            </div>
            if layout.show_footer {
                <Footer />
            }
        </>
    }
}

fn header_buttons(buttons: &[HeaderBtn], show_menu: UseToggleHandle<bool>) -> Html {
    let has_label = buttons.iter().any(|b| b.label.is_some());

    html! {
        <span>
            { for buttons.iter().map(|btn| html!{
                <HeaderButton btn={(*btn).clone()} {has_label} show_menu={show_menu.clone()} />
            }) }
        </span>
    }
}
