use super::{PublicRoute, root_switch};
use crate::{
    context::{Session, SessionAction},
    hooks::use_visibility,
    layouts::{SinglePane, TwoPane},
    services::requests,
};
use gloo::utils::window;
use gloo_events::EventListener;
use gloo_utils::format::JsValueSerdeExt;
use serde::Serialize;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Serialize)]
struct CheckUpdateMsg {
    #[serde(rename = "type")]
    msg_type: String,
    token: String,
}

fn viewport_width() -> f64 {
    window()
        .inner_width()
        .ok()
        .and_then(|width| width.as_f64())
        .unwrap_or_default()
}

#[function_component(AppLayout)]
pub fn app_layout() -> Html {
    let width = use_state_eq(viewport_width);
    let visibility = use_visibility();
    let session = use_context::<Session>().expect("SessionState context not found");

    {
        let width = width.clone();
        use_effect_with((), move |_| {
            let resize_listener = {
                let width = width.clone();
                EventListener::new(&window(), "resize", move |_| {
                    width.set(viewport_width());
                })
            };

            let orientation_listener = {
                let width = width.clone();
                EventListener::new(&window(), "orientationchange", move |_| {
                    width.set(viewport_width());
                })
            };

            || {
                drop(resize_listener);
                drop(orientation_listener);
            }
        });
    }

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
        if *width >= 1024.0 {
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
