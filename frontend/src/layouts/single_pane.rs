use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(SinglePane)]
pub fn single_pane(props: &Props) -> Html {
    html! { { props.children.clone() } }
}
