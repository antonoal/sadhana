use yew::{html::ChildrenProps, prelude::*};

#[derive(Clone, PartialEq)]
pub struct DataRefresh {
    pub version: u64,
    pub notify_saved: Callback<()>,
}

#[function_component(DataRefreshProvider)]
pub fn data_refresh_provider(props: &ChildrenProps) -> Html {
    let version = use_state(|| 0_u64);

    let notify_saved = {
        let version = version.clone();
        Callback::from(move |_| {
            version.set(*version + 1);
        })
    };

    let ctx = DataRefresh {
        version: *version,
        notify_saved,
    };

    html! {
        <ContextProvider<DataRefresh> context={ctx}>
            { props.children.clone() }
        </ContextProvider<DataRefresh>>
    }
}
