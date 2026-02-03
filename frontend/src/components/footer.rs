use crate::{css::*, routes::AppRoute};
use tw_merge::*;
use yew::prelude::*;
use yew_router::hooks::use_route;
use yew_router::prelude::*;

const FOOTER_ITEMS: [(AppRoute, &str); 4] = [
    (AppRoute::Default, "icon-home"),
    (AppRoute::Charts, "icon-graph"),
    (AppRoute::Yatras, "icon-user-group"),
    (AppRoute::Settings, "icon-adjust"),
];

#[function_component(Footer)]
pub fn footer() -> Html {
    let route = use_route::<AppRoute>().unwrap_or_default();

    html! {
        <div
            id="footer"
            class="fixed bottom-0 left-0 z-10 w-full h-16 bg-white/50 border-t border-zinc-200/50 dark:bg-zinc-700/50  dark:border-zinc-700/50"
        >
            <div class="bg-transparent justify-center">
                <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                    <div class="relative px-8 sm:rounded-3xl sm:px-20">
                        <div class={MENU_CSS}>
                            { for FOOTER_ITEMS.iter().map(|(path, icon_css)| {
                                let icon_css = if route == *path {
                                    format!("{}-solid !text-amber-500", icon_css)
                                } else {
                                    icon_css.to_string()
                                };
                                html! {
                                    <span>
                                        <Link<AppRoute> to={path.clone()}>
                                            <i class={tw_merge!(icon_css, FOOTER_ICON_CSS)} />
                                        </Link<AppRoute>>
                                    </span>
                                }
                                }) }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
