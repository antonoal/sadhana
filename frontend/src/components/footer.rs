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

const FOOTER_ITEMS_NO_HOME: [(AppRoute, &str); 3] = [
    (AppRoute::Charts, "icon-graph"),
    (AppRoute::Yatras, "icon-user-group"),
    (AppRoute::Settings, "icon-adjust"),
];

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(true)]
    pub show_home: bool,
    #[prop_or(false)]
    pub inside_parent: bool,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    let route = use_route::<AppRoute>().unwrap_or_default();
    let footer_items = if props.show_home {
        &FOOTER_ITEMS[..]
    } else {
        &FOOTER_ITEMS_NO_HOME[..]
    };

    html! {
        <div
            id="footer"
            class={tw_merge!(
                "bottom-0 z-10 w-full h-16 bg-white/50 border-t border-zinc-200/50 dark:bg-zinc-700/50 dark:border-zinc-700/50",
                if props.inside_parent {
                    "absolute left-0"
                } else {
                    "fixed left-0"
                }
            )}
        >
            <div class="bg-transparent justify-center">
                <div class="relative py-3 sm:max-w-xl sm:mx-auto">
                    <div class="relative px-8 sm:rounded-3xl sm:px-20">
                        <div class={MENU_CSS}>
                            { for footer_items.iter().map(|(path, icon_css)| {
                                let is_active = route == *path
                                    || route.is_child_of(path)
                                    || (!props.show_home
                                        && route == AppRoute::Default
                                        && *path == AppRoute::Charts);

                                let icon_css = if is_active {
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
