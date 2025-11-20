use leptos::prelude::*;
use tw_merge::*;

use crate::{components::footer::Footer, layouts::LayoutContext};

#[component]
pub fn MobileLayout(children: Children) -> impl IntoView {
    let online = true;
    let layout = use_context::<LayoutContext>().expect("Layout context it not provided");

    view! {
        <Show when=move || !online >
            <div class="absolute bg-red-500 w-full h-4 top-[env(safe-area-inset-top)] z-10 overscroll-none">
                <p class="text-white text-center overflow-hidden text-xs">
                    // {Locale::current().offline_msg()}
                    {"Offline!"}
                </p>
            </div>
        </Show>
        <div
            id="content"
            class=move || {
                tw_merge!(
                    "fixed pt-safe-top top-0 {} left-0 right-0 overflow-y-auto",
                    if !layout.hide_footer.get() {"bottom-16"} else {"bottom-0"},
                    if !online {"top-4"} else {""}
                    )
                }
        >
            // 100vh-4rem means screen minus bottom-16; env(...) - the height of iPhone notch
            <div class="bg-transparent min-h-[calc(100vh-4rem-env(safe-area-inset-top))] justify-center items-center py-[calc(0.5rem-env(safe-area-inset-top))] sm:py-[calc(3rem-env(safe-area-inset-top))]">
                <div class="w-full text-center relative">
                    <div class="absolute flex w-full h-full flex-col justify-center px-4">
                        <div class="relative">
                            <div class="relative sm:max-w-md md:max-w-md lg:max-w-lg xl:max-w-lg 2xl:max-w-lg mx-auto">
                                <div class="relative flex justify-between py-10">
                                    // {header_button(&left_buttons, nav.clone(), show_ctx_menu.clone())}
                                    // {header_button(&right_buttons, nav.clone(), show_ctx_menu.clone())}
                                </div>
                            </div>
                        </div>
                    </div>
                    <img class="logo h-20 inline-block" src="/images/logo.png" />
                </div>
                <div class="relative sm:max-w-xl md:max-w-3xl lg:max-w-4xl xl:max-w-5xl 2xl:max-w-7xl mx-auto">
                    <div class="relative px-4 py-4 rounded-3xl sm:px-20 md:px-20 lg:px-20 xl:px-30 2xl:px-30">
                        <Show when=move || layout.header_title.get().is_some() >

                                        <div class="pb-5 text-center">
                                            <h5 class="text-xl font-medium text-zinc-500 dark:text-zinc-100">
                                                {layout.header_title.get()}
                                            </h5>
                                //             {for props.header_sub_label.iter().map(|sl| {
                                    //                 html!{<span class="text-sm text-zinc-300 dark:text-zinc-200">{sl}</span>}
                                    //             })}
                                            </div>
                        </Show>
                        // if *show_month_cal {
                        //     <MonthCalendar
                        //         close={month_cal_toggle.clone()}
                        //         highlight_incomplete_dates={props.calendar.as_ref().map(|cal| cal.highlight_incomplete_dates).unwrap_or(false)}
                        //         />
                        // }
                        // if let Some(cal) = props.calendar.as_ref() {
                        //     <Calendar
                        //         highlight_incomplete_dates={cal.highlight_incomplete_dates}
                        //         selected_date_incomplete={cal.selected_date_incomplete}
                        //         />
                        // }
                        {children()}
                    </div>
                </div>
            </div>
        </div>
        <Show when=move || !layout.hide_footer.get() >
            <Footer />
        </Show>
    }
}
