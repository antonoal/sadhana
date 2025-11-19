pub mod mobile_layout;

use leptos::prelude::*;
use leptos_router::components::Outlet;
use mobile_layout::MobileLayout;

#[component]
pub fn AppLayout() -> impl IntoView {
    // Placeholder for future switch between mobile and table layouts
    let is_mobile = true;
    let (show_footer, set_show_footer) = signal(true);
    let (header_label, set_header_label) = signal(None);

    view! {
        {move || {
            if is_mobile {
                view! { <MobileLayout show_footer header_label><Outlet/></MobileLayout> }
            } else {
                view! { <MobileLayout show_footer header_label><Outlet/></MobileLayout> }
            }
        }}
    }
}
