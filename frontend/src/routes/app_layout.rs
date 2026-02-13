use super::{PublicRoute, root_switch};
use crate::{
    context::{Session, SessionAction},
    hooks::use_visibility,
    layouts::{SinglePane, TwoPane},
    services::requests,
};
use gloo::utils::window;
use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::use_window_size;
use yew_router::prelude::*;

#[derive(Debug, Serialize)]
struct CheckUpdateMsg {
    #[serde(rename = "type")]
    msg_type: String,
    token: String,
}

#[function_component(AppLayout)]
pub fn app_layout() -> Html {
    let (width, _) = use_window_size();
    let visibility = use_visibility();
    let session = use_context::<Session>().expect("SessionState context not found");

    {
        // On wake of the app check for the app update and update today
        let session = session.clone();
        use_effect_with(visibility.clone(), move |v| {
            if v.visible {
                session.dispatch(SessionAction::UpdateToday);
                if let Some(token) = requests::get_token() {
                    if let Some(controller) = window().navigator().service_worker().controller() {
                        let msg = CheckUpdateMsg {
                            msg_type: "CHECK_UPDATE".into(),
                            token,
                        };
                        let msg = JsValue::from_serde(&msg)
                            .expect("Failed to serialize CHECK_UPDATE message");
                        controller.post_message(&msg).ok();
                    }
                }
            }
        });
    }

    html! {
        if false /*&& width >= 1024.0*/{
            <TwoPane>
                <Switch<PublicRoute> render={|route| root_switch(route, false)} />
            </TwoPane>
        } else {
            <SinglePane>
                <Switch<PublicRoute> render={|route| root_switch(route, true)} />
            </SinglePane>
        }
    }
}
