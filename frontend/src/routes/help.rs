use common::error::AppError;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{blank_page::BlankPage, list_errors::ListErrors},
    css::*,
    hooks::use_user_context,
    i18n::*,
    model::*,
    services, BaseRoute,
};

#[function_component(Help)]
pub fn help() -> Html {
    html! {
              <BlankPage show_footer=true >
        <div class={ BODY_DIV_CSS }>
        <div class="text-center">
      <h5 class="mb-4 text-xl font-medium leading-tight">{"FAQ"}</h5>
      <p class="text-zinc-500 dark:text-zinc-200">{"How to Reset or Update Your Password?
      How to Update Your Name?
      How to use groups?
      How to update my data?
      How to add new person to the group?
      How to Log In to Your Account?"}</p>
    </div>
        </div>
              </BlankPage>
            }
}