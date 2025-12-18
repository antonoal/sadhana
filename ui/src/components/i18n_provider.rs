use leptos::prelude::*;
use leptos_fluent::leptos_fluent;

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en",
        set_language_to_local_storage: true,
        initial_language_from_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_local_storage: true,
        check_translations: "./src/**/*.rs",
    }
}
