use crate::css::*;
use leptos::prelude::*;
use leptos_router::{components::*, hooks::use_location};
use tw_merge::*;

const FOOTER_ITEMS: [(&str, &str); 4] = [
    ("/", "icon-home"),
    ("/charts", "icon-graph"),
    ("/yatras", "icon-user-group"),
    ("/settings", "icon-adjust"),
];

#[component]
pub fn Footer() -> impl IntoView {
    let location = use_location();

    {
        view! {
            <div id="footer" class="fixed bottom-0 left-0 z-10 w-full h-16 bg-white/50 border-t border-zinc-200/50 dark:bg-zinc-700/50  dark:border-zinc-700/50">
                <div class="bg-transparent justify-center">
                    <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                        <div class="relative px-8 sm:rounded-3xl sm:px-20">
                            <div class=MENU_CSS>
                                {FOOTER_ITEMS.iter().map(|(path, icon_css)| {
                                    let css = move || {
                                        tw_merge!(
                                            icon_css,
                                            FOOTER_ICON_CSS,
                                            if location.pathname.get() == *path {
                                                "-solid !text-amber-500"
                                            } else {
                                                ""
                                            }
                                        )
                                    };
                                    view! {
                                        <span>
                                            <A href=path.to_string()>
                                                <i class=css />
                                            </A>
                                        </span>
                                    }
                                    }).collect_view()}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
