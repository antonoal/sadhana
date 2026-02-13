use crate::{
    css::*,
    hooks::use_layout_ctx,
    i18n::{DEFAULT_LANGUAGE_KEY, LANGUAGE_DATA, Locale, USER_LANGUAGE_STORAGE_KEY},
    routes::AppRoute,
    tr,
};
use gloo::storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_mount;

#[function_component(Language)]
pub fn language() -> Html {
    let layout = use_layout_ctx();
    let stored_language = LocalStorage::get::<String>(USER_LANGUAGE_STORAGE_KEY)
        .unwrap_or(DEFAULT_LANGUAGE_KEY.to_owned());

    let is_checked_lang = |s: &str| -> bool { stored_language == s };

    let reload = use_force_update();

    let language_onchange = {
        let reload = reload.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();

            let input: HtmlInputElement = e.target_unchecked_into();

            if input.value() == DEFAULT_LANGUAGE_KEY {
                LocalStorage::delete(USER_LANGUAGE_STORAGE_KEY);
            } else {
                LocalStorage::set(USER_LANGUAGE_STORAGE_KEY, input.value()).unwrap();
            }

            reload.force_update();
        })
    };

    {
        let layout = layout.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                true,
                Some(tr!(language)),
                Some(AppRoute::Settings),
                vec![],
            );
        });
    }

    html! {
        <div class={BODY_DIV_CSS}>
            <div class="relative">
                <select class={INPUT_CSS} id="language" onchange={language_onchange} required=true>
                    <option
                        class="text-black"
                        value={DEFAULT_LANGUAGE_KEY}
                        selected={is_checked_lang(DEFAULT_LANGUAGE_KEY)}
                    >
                        { Locale::current().default_language().as_str() }
                    </option>
                    { LANGUAGE_DATA
                            .iter()
                            .map(|(s, s_full)| html! {
                                <option class={ "text-black" } value={ s.to_owned() } selected={ is_checked_lang(s) }>{ s_full }</option>
                            })
                            .collect::<Html>() }
                </select>
                <label for="language" class={INPUT_LABEL_CSS}>
                    <i class="icon-lang" />
                    { format!(" {}: ", tr!(language)) }
                </label>
            </div>
        </div>
    }
}
