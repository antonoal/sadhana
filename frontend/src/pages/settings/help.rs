use tw_merge::*;
use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::use_navigator;

use crate::{
    components::summary_details::SummaryDetails,
    css::*,
    hooks::{use_cache_aware_async, use_layout_ctx, use_user_ctx},
    routes::AppRoute,
    services::get_version,
    tr,
};

#[function_component(Help)]
pub fn help() -> Html {
    let layout = use_layout_ctx();
    let nav = use_navigator().unwrap();
    let ctx = use_user_ctx();
    let api_version = use_cache_aware_async(get_version());

    {
        let version = api_version.clone();
        let layout = layout.clone();
        let show_footer = ctx.is_authenticated();
        use_mount(move || {
            layout.set_app_service_layout(show_footer, None);
            version.run();
        })
    }

    let send_msg_onclick = {
        let nav = nav.clone();
        Callback::from(move |_: MouseEvent| {
            nav.push(&AppRoute::SupportForm);
        })
    };

    let yt_links = [
        (tr!(help_registration), "Hw1DQ3sRNAk?si=lpVPuUCQp8j-xJmC"),
        (tr!(help_ios_web_app), "KBViu8I4cJI?si=j-PwU1VGld1Yoa6z"),
        (tr!(help_add_practice), "cbQ5aVXvXiU?si=dPGawgCnPL8C1yzo"),
        (tr!(help_rename_practice), "jVfngYlbA68?si=xwMbf4WgtnGih5bj"),
        (
            tr!(help_add_graph_report),
            "gJ9jqB-nGtg?si=kNgEOfzfgWE99wco",
        ),
        (tr!(help_graph_show_avg), "qqLOm_HZYWk?si=pOLdH4lBKoiYkjvY"),
        (
            tr!(help_graph_add_practice),
            "WY8LUyf_NaM?si=FxLay9PK9EDzXYlL",
        ),
        (
            tr!(help_graph_bar_chart_layouts),
            "QbW1nANFX-w?si=fmBqDzuncfP0XlfU",
        ),
        (
            tr!(help_add_table_report),
            "Bg8eAmoT-_I?si=RNpD3jYqs8RKxSjH",
        ),
    ];

    html! {
        <>
            <div class={BODY_DIV_SPACE_10_CSS}>
                <div class="text-center">
                    <h5 class="mb-4 text-xl font-medium leading-tight">{ tr!(help_faq) }</h5>
                    { for yt_links.iter().map(|(title, link)| html!{
                        <SummaryDetails label={title.to_string()}>
                            <div class="aspect-video">
                                <iframe
                                    class="w-full h-full"
                                    src={format!("https://www.youtube.com/embed/{link}")}
                                    frameborder="0"
                                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                    referrerpolicy="strict-origin-when-cross-origin"
                                    allowfullscreen=true
                                    />
                            </div>
                        </SummaryDetails>
                    }) }
                </div>
            </div>
            if ctx.is_authenticated() {
                <div class="pt-8">
                    <div class={TWO_COLS_CSS}>
                        <div class="relative">
                            <button onclick={send_msg_onclick} class={BTN_CSS_NO_MARGIN}>
                                <i class="icon-mail" />
                                { tr!(sf_send_us_message) }
                            </button>
                        </div>
                        <div class="relative">
                            <a
                                target="_blank"
                                rel="noopener noreferrer"
                                href="https://t.me/sadhanapro"
                                class={tw_merge!("flex justify-center", BTN_CSS_NO_MARGIN)}
                            >
                                <i class="icon-send" />
                                { tr!(sf_follow_on_telegram) }
                            </a>
                        </div>
                    </div>
                </div>
            }
            <div
                class={tw_merge!(BODY_DIV_SPACE_10_CSS, "text-center text-sm")}
            >
                <label>
                    { format!(
                        "{} (Git hash)",
                        api_version.data.clone().map(|info| info.git_sha).unwrap_or_default()) }
                </label>
            </div>
        </>
    }
}
