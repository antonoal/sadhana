use tw_merge::*;
use yew::prelude::*;
use yew_hooks::UseToggleHandle;
use yew_router::hooks::use_navigator;

use crate::{
    context::{Action, CtxMenuEntry, HeaderButton as HeaderBtn},
    css::POPUP_BG_CSS,
};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
    pub has_label: bool,
    pub show_menu: UseToggleHandle<bool>,
    pub btn: HeaderBtn,
}
#[function_component(HeaderButton)]
pub fn header_button(props: &Props) -> Html {
    let nav = use_navigator().unwrap();

    let css = tw_merge!(
        "no-underline text-amber-400",
        if props.has_label {
            "text-base font-bold"
        } else {
            "text-xl"
        }
    );

    let hide_menu = {
        let menu_toggle = props.show_menu.clone();
        Callback::from(move |_| menu_toggle.set(false))
    };

    let onclick = |action: &Action| {
        let nav = nav.clone();
        let show_menu = props.show_menu.clone();

        match action {
            Action::Cb(cb) => cb.clone(),
            Action::Redirect(to) => {
                let route = to.clone();
                Callback::from(move |_| nav.push(&route))
            }
            Action::NavBack => Callback::from(move |_| nav.back()),
            Action::CtxMenu(_) => Callback::from(move |_| show_menu.toggle()),
        }
    };

    let onclick_with_hide = |action: &Action| {
        let hide_menu = hide_menu.clone();
        let onclick = onclick(action);
        Callback::from(move |e: MouseEvent| {
            hide_menu.emit(e.clone());
            onclick.emit(e)
        })
    };

    let ctx_menu_iter_html = |item: &CtxMenuEntry| match &item.action {
        Action::Cb(_) | Action::Redirect(_) => html! {
            <li onclick={onclick_with_hide(&item.action)}>
                <div
                    class="flex px-2 py-2 text-sm hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
                >
                    <label>
                        <i
                            class={tw_merge!(item.icon_css.to_owned().unwrap_or_default(), "flex-shrink-0 w-5")}
                        />
                        { &item.label }
                    </label>
                </div>
            </li>
        },
        _ => panic!("Unsupported feature - nested context menus"),
    };

    let ctx_menu_items = |action: &Action| match action {
        Action::CtxMenu(items) => items.iter().map(ctx_menu_iter_html).collect::<Html>(),
        _ => html! {},
    };

    html! {
        <>
            <button
                type={props.btn.btn_type.to_string()}
                class={css.clone()}
                onclick={onclick(&props.btn.action)}
            >
                <i class={props.btn.icon_css.to_owned().unwrap_or_default()} />
                if let Some(l) = &props.btn.label {
                    { l }
                }
            </button>
            if *props.show_menu && matches!(&props.btn.action, Action::CtxMenu(_)) {
                <div
                    // Fill on the screen with a div that hides menu on click
                    class="fixed top-0 bottom-0 left-0 right-0 w-full h-full z-10"
                    onclick={let show_menu = props.show_menu.clone();
                    Callback::from(move |_| show_menu.toggle())}
                />
                <ul
                    class={tw_merge!("origin-top-right absolute right-0 w-65 text-gray-800 dark:text-white focus:outline-none z-20 mt-2 py-1", POPUP_BG_CSS)}
                >
                    { ctx_menu_items(&props.btn.action) }
                </ul>
            }
        </>
    }
}
