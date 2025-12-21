use leptos::prelude::*;
use leptos_fluent::I18n;

pub mod confirmation;
pub mod login;
pub mod pwd_reset;
pub mod register;

fn about_url() -> String {
    let i18n = expect_context::<I18n>();
    format!("https://sadhana.pro/{}", i18n.language.get().id)
}
