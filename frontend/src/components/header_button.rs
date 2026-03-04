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
    let anchor_ref = use_node_ref();
    let menu_ref = use_node_ref();
    let align_right = use_state(|| false);

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

    {
        let anchor_ref = anchor_ref.clone();
        let menu_ref = menu_ref.clone();
        let align_right = align_right.clone();
        let show_menu = *props.show_menu;
        let is_ctx_menu = matches!(&props.btn.action, Action::CtxMenu(_));

        use_effect_with((show_menu, is_ctx_menu), move |(show_menu, is_ctx_menu)| {
            if !*show_menu || !*is_ctx_menu {
                align_right.set(false);
            } else {
                align_right.set(false);
                if let (Some(anchor_el), Some(menu_el), Some(window)) = (
                    anchor_ref.cast::<web_sys::Element>(),
                    menu_ref.cast::<web_sys::Element>(),
                    web_sys::window(),
                ) {
                    if let Ok(viewport_width) = window.inner_width() {
                        if let Some(viewport_width) = viewport_width.as_f64() {
                            let anchor_rect = anchor_el.get_bounding_client_rect();
                            let menu_rect = menu_el.get_bounding_client_rect();
                            let projected_right = anchor_rect.left() + menu_rect.width();
                            align_right.set(projected_right > viewport_width);
                        }
                    }
                }
            }

            || ()
        });
    }

    html! {
        <span class="relative inline-flex items-center" ref={anchor_ref}>
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
                    ref={menu_ref}
                    class={tw_merge!(
                        "absolute top-full mt-2 w-72 text-gray-800 dark:text-white focus:outline-none z-20 py-1",
                        if *align_right {
                            "origin-top-right right-0"
                        } else {
                            "origin-top-left left-0"
                        },
                        POPUP_BG_CSS
                    )}
                >
                    { ctx_menu_items(&props.btn.action) }
                </ul>
            }
        </span>
    }
}
