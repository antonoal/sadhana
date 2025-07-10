use common::error::AppError;
use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::model::auth::{UserInfo, UserInfoWrapper};
use crate::routes::{auth::login::Login, home::Home};
use crate::services::requests::request_api_get;

mod css;
mod model;
mod routes;
mod services;

#[component]
fn App() -> impl IntoView {
    let (user_ctx, set_user_ctx) = signal(UserInfo::default());
    let saved_user = LocalResource::new(async move || current().await.map(|wrapper| wrapper.user));

    provide_context(user_ctx);

    view! {
      <Router>
        <Routes fallback=|| "🤷‍♂️ Not found.">
          <Route path=path!("/login") view=move || view! { <Login set_user_ctx />}/>
          <ParentRoute path=path!("") view=move || view! {
            <Suspense fallback=|| view! { <p>"Loading current user..."</p> }>
              {move || Suspend::new(async move {
                let user = saved_user.await;
                view! {
                  <Show when=move || { !user.as_ref().is_ok_and(|user_info| user_info.is_authenticated()) }>
                    <Redirect path="/login"/>
                  </Show>
                }
              })}
            </Suspense>
            <Outlet/>
            }>
            <Route path=path!("/") view=Home/>
          </ParentRoute>
        </Routes>
      </Router>
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
