use common::error::AppError;
use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::components::background::Background;
use crate::layouts::AppLayout;
use crate::model::auth::{UserInfo, UserInfoWrapper};
use crate::pages::charts::Charts;
use crate::pages::{auth::login::Login, home::Home};
use crate::services::requests::request_api_get;

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

    view! {
      <main>
        <Background />
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

#[component]
fn ProtectedRoutes(
    user_ctx: ReadSignal<UserInfo>,
    set_user_ctx: WriteSignal<UserInfo>,
) -> impl IntoView {
    let saved_user = LocalResource::new(async move || current().await.map(|wrapper| wrapper.user));

    view! {
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
    }
}
