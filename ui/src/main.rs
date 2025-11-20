use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::components::i18n_provider::I18nProvider;
use crate::components::loading_context::LoadingContext;
use crate::components::loading_overlay::LoadingOverlay;
use crate::components::{background::Background, protected_routes::ProtectedRoutes};
use crate::layouts::AppLayout;
use crate::model::auth::UserInfo;
use crate::pages::charts::Charts;
use crate::pages::{auth::login::Login, home::Home};

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
              <ParentRoute path=path!("") view=move || view! { <ProtectedRoutes user_ctx set_user_ctx/> } >
                <ParentRoute path=path!("") view=AppLayout >
                  <Route path=path!("/") view=move || view! { <Home /> } />
                  <Route path=path!("/charts") view=Charts />
                </ParentRoute>
              </ParentRoute>
              <Route path=path!("/login") view=move || view! { <Login set_user_ctx />}/>
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
