use crate::{
    model::auth::{UserInfo, UserInfoWrapper},
    services::requests::request_api_get,
};
use common::error::AppError;
use leptos::prelude::*;
use leptos_router::components::{Outlet, Redirect};

#[component]
pub fn ProtectedRoutes(
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

/// Get current user info
async fn current() -> Result<UserInfoWrapper, AppError> {
    request_api_get("/user".to_string()).await
}
