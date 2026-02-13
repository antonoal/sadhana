use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::use_navigator;

use crate::{
    css::*,
    hooks::{use_async_with_error, use_cache_aware_async, use_layout_ctx},
    routes::AppRoute,
    services, tr,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub yatra_id: AttrValue,
}

#[function_component(JoinYatra)]
pub fn join_yatra(props: &Props) -> Html {
    let layout = use_layout_ctx();
    let yatra = use_cache_aware_async(services::get_yatra(&props.yatra_id).map(|resp| resp.yatra));
    let nav = use_navigator().unwrap();

    let join = {
        let yatra_id = props.yatra_id.clone();
        let nav = nav.clone();
        use_async_with_error(async move {
            services::join_yatra(&yatra_id)
                .await
                .map(|_| nav.push(&AppRoute::Yatras))
        })
    };

    {
        let layout = layout.clone();
        let yatra = yatra.clone();
        use_mount(move || {
            layout.set_app_service_layout(
                true,
                Some(tr!(yatra_join)),
                Some(AppRoute::Yatras),
                vec![],
            );
            yatra.run();
        });
    }

    {
        let layout = layout.clone();
        let yatra = yatra.clone();
        use_effect_with(yatra, move |y| {
            let title = y
                .data
                .as_ref()
                .map(|v| format!("{} {}", tr!(yatra_join), v.name));
            layout.set_app_service_layout(true, title, Some(AppRoute::Yatras), vec![]);
            || ()
        });
    }

    let onsubmit = {
        let join = join.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            join.run();
        })
    };

    html! {
        <div class={BODY_DIV_CSS}>
            <form {onsubmit}>
                <div class="relative">
                    <button class={SUBMIT_BTN_CSS}>
                        <i class="icon-tick" />
                        { format!(" {}", tr!(yatra_join)) }
                    </button>
                </div>
            </form>
        </div>
    }
}
