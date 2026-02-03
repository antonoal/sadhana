// Fix for now: https://github.com/rustwasm/wasm-bindgen/issues/2774
// #![allow(clippy::unused_unit)]

use yew::prelude::*;
use yew_router::prelude::*;

use components::{Background, LoadingOverlay};
use context::*;
use routes::*;

mod components;
mod context;
mod css;
mod hooks;
mod i18n;
mod layouts;
mod model;
mod pages;
mod routes;
mod services;
mod utils;
mod web_sys_ext;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Background />
            <LoadingContextProvider>
                <LoadingOverlay />
                <NetworkStatusProvider>
                    <AppUpdateContextProvider>
                        <ErrorContextProvider>
                            <UserContextProvider>
                                <SessionStateProvider>
                                    <LayoutStateProvider>
                                        <AppLayout />
                                    </LayoutStateProvider>
                                </SessionStateProvider>
                            </UserContextProvider>
                        </ErrorContextProvider>
                    </AppUpdateContextProvider>
                </NetworkStatusProvider>
            </LoadingContextProvider>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
