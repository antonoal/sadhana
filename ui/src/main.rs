use common::error::AppError;
use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::components::background::Background;
use crate::model::auth::{UserInfo, UserInfoWrapper};
use crate::routes::charts::Charts;
use crate::routes::{auth::login::Login, home::Home};
use crate::services::requests::request_api_get;

mod components;
mod css;
mod model;
mod routes;
mod services;

#[component]
fn App() -> impl IntoView {
    let (user_ctx, set_user_ctx) = signal(UserInfo::default());
    let (show_footer, set_show_footer) = signal(true);
    let (header_label, set_header_label) = signal(None);
    let saved_user = LocalResource::new(async move || current().await.map(|wrapper| wrapper.user));

    provide_context(user_ctx);

    view! {
      <main>
        <Background />
        <Router>
          <Routes fallback=|| "🤷‍♂️ Not found.">
            <ParentRoute path=path!("") view=move || view! {
              <Suspense fallback=|| view! { <p>"Loading current user..."</p> } >
                {move || Suspend::new(async move {
                  if !user_ctx.get().is_authenticated() {
                    log::debug!("Fetching user from server");
                    if let Ok(user) = saved_user.await {
                      set_user_ctx.set(user);
                    }
                  }
                  view! {
                    <Show
                      when=move || user_ctx.get().is_authenticated()
                      fallback=move || view! { <Redirect path="/login" /> }
                    >
                      <Outlet/>
                    </Show>
                  }
                })}
              </Suspense>
            } >
              <Route path=path!("") view=move || view! { <Home show_footer=show_footer header_label=header_label/> } />
              <Route path=path!("/charts") view=Charts />
            </ParentRoute>
            <Route path=path!("/login") view=move || view! { <Login set_user_ctx />}/>
          </Routes>
        </Router>
      </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App/> })
}

/// Get current user info
async fn current() -> Result<UserInfoWrapper, AppError> {
    request_api_get("/user".to_string()).await
}
