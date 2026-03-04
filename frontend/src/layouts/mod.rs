use yew::prelude::*;
use yew_hooks::UseToggleHandle;

use crate::{components::HeaderButton, context::HeaderButton as HeaderBtn};

mod single_pane;
mod two_pane;

pub use single_pane::*;
pub use two_pane::*;

pub(super) fn pane_offline_banner(message: String) -> Html {
    html! {
        <div class="absolute bg-red-500 w-full h-4 top-[env(safe-area-inset-top)] z-10 overscroll-none">
            <p class="text-white text-center overflow-hidden text-xs">
                { message }
            </p>
        </div>
    }
}

pub(super) fn pane_header_buttons(buttons: &[HeaderBtn], show_menu: UseToggleHandle<bool>) -> Html {
    let has_label = buttons.iter().any(|b| b.label.is_some());

    html! {
        <span>
            { for buttons.iter().map(|btn| html!{
                <HeaderButton btn={(*btn).clone()} {has_label} show_menu={show_menu.clone()} />
            }) }
        </span>
    }
}
