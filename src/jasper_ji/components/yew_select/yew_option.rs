use super::msg_ctx::MessageContext;

use yew::prelude::*;
use gloo_console::log;


#[derive(Clone, PartialEq, Properties)]
pub struct YewOptionProps {
    #[prop_or_default]
    pub label: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub selected:bool
}

#[function_component]
pub fn YewOption(props: &YewOptionProps) -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();

    let l = props.label.clone();
    let ll = props.label.clone();
    let onclick = Callback::from(move |_| {
        msg_ctx.dispatch(l.clone());
    });

    let mut classes = vec!["el-select-dropdown__item".to_string()];
    if props.selected {
        classes.push("selected".to_string());
    }
    html! {
        <li
            class={classes!(classes)}
            {onclick}
        >
            {ll.clone()}
        </li>
    }
}