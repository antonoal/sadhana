use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::{
    components::{
        background::Background, i18n_provider::I18nProvider, loading_context::LoadingContext,
        loading_overlay::LoadingOverlay, protected_routes::ProtectedRoutes,
    },
    layouts::AppLayout,
    model::auth::UserInfo,
    pages::{
        auth::{login::Login, register::Register},
        charts::Charts,
        home::Home,
    },
};

mod components;
mod css;
mod layouts;
mod model;
mod pages;
mod services;

#[component]
fn App() -> impl IntoView {
    let (user_ctx, set_user_ctx) = signal(UserInfo::default());

    provide_context(user_ctx);

    LoadingContext::provide();

    view! {
      <main>
        <Background />
        <LoadingOverlay />
        <I18nProvider>
          <Router>
            <Routes fallback=|| "🤷‍♂️ Not found.">
              <ParentRoute path=path!("") view=AppLayout >
                <ParentRoute path=path!("") view=move || view! { <ProtectedRoutes user_ctx set_user_ctx/> } >
                  <Route path=path!("/") view=move || view! { <Home /> } />
                  <Route path=path!("/charts") view=Charts />
                </ParentRoute>
                <Route path=path!("/login") view=move || view! { <Login set_user_ctx />} />
                <Route path=path!("/register") view=move || view! { <Register />} />
              </ParentRoute>
            </Routes>
          </Router>
        </I18nProvider>
      </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> })
}
