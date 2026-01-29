use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(TwoPane)]
pub fn two_pane(props: &Props) -> Html {
    html! { { props.children.clone() } }
}
