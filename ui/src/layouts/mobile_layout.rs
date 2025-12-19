use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use tw_merge::*;

use crate::{
    components::{errors::ErrorBanner, footer::Footer},
    layouts::{ButtonAction, HeaderButton, LayoutContext},
};

pub const HEADER_BUTTON_CSS: &str = "no-underline text-amber-400";

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
                                    {move || {
                                        let buttons = layout.header_buttons.read_only().get();
                                        header_button(buttons.left)}}
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
                        <ErrorBanner />
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

fn header_button(buttons: Vec<HeaderButton>) -> impl IntoView {
    let nav = use_navigate();
    // fn header_button(
    // buttons: &[&HeaderButtonProps],
    // nav: Navigator,
    // show_menu: UseToggleHandle<bool>,
    // ) -> Html {
    let css = tw_merge!(
        HEADER_BUTTON_CSS,
        if buttons.iter().any(|p| p.label.is_some()) {
            "text-base font-bold"
        } else {
            "text-xl"
        }
    );

    // let hide_menu = {
    //     let menu_toggle = show_menu.clone();
    //     Callback::from(move |_| menu_toggle.set(false))
    // };

    let onclick = |action: &ButtonAction| {
        // let nav = nav.clone();
        // let show_menu = show_menu.clone();

        match action {
            ButtonAction::Cb(cb) => todo!(), //cb.clone(),
            ButtonAction::Navigate(url) => nav(url, Default::default()),
            // Action::Redirect(to) => {
            //     let route = to.clone();
            //     Callback::from(move |_| nav.push(&route))
            // }
            // Action::CtxMenu(_) => Callback::from(move |_| show_menu.toggle()),
        }
    };

    // let onclick_with_hide = |action: &Action| {
    //     let hide_menu = hide_menu.clone();
    //     let onclick = onclick(action);
    //     Callback::from(move |e: MouseEvent| {
    //         hide_menu.emit(e.clone());
    //         onclick.emit(e)
    //     })
    // };

    // let ctx_menu_iter_html = |item: &CtxMenuEntry| match &item.action {
    //     Action::Cb(_) | Action::Redirect(_) => html! {
    //         <li onclick={onclick_with_hide(&item.action)}>
    //             <div class="flex px-2 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg">
    //                 <label>
    //                     <i class={tw_merge!(item.icon_css.to_owned().unwrap_or_default(), "flex-shrink-0 w-5")} />
    //                     {&item.label}
    //                 </label>
    //             </div>
    //         </li>
    //     },
    //     _ => panic!("Unsupported feature - nested context menus"),
    // };

    // let ctx_menu_items = |action: &Action| match action {
    //     Action::CtxMenu(items) => items.iter().map(ctx_menu_iter_html).collect::<Html>(),
    //     _ => html! {},
    // };

    view! {
        <span>
            {buttons.into_iter().map(|btn| {
                // clone small pieces we need so the view doesn't borrow from the local `btn`
                let btn_type = btn.btn_type.clone();
                let icon = btn.icon.clone().unwrap_or_default();
                let label = btn.label.clone().unwrap_or_default();

                view! {
                    <button
                        type={btn_type.as_str()}
                        class={css.clone()}
                        // onclick={onclick(&props.action)}
                    >
                        <i class={icon}></i>
                        {label}
                    </button>
                    // if *show_menu && matches!(&props.action, Action::CtxMenu(_)) {
                    //     <div // Fill on the screen with a div that hides menu on click
                    //         class="fixed top-0 bottom-0 left-0 right-0 w-full h-full z-10"
                    //         onclick={
                    //             let show_menu = show_menu.clone();
                    //             Callback::from(move |_| show_menu.toggle())
                    //         }
                    //     />
                    //     <ul
                    //         class={tw_merge!("origin-top-right absolute right-0 w-65 text-gray-800 dark:text-white focus:outline-none z-20 mt-2 py-1", POPUP_BG_CSS)}
                    //     >
                    //         {ctx_menu_items(&props.action)}
                    //     </ul>
                    // }
                }
            }).collect_view()}
        </span>
    }
}
