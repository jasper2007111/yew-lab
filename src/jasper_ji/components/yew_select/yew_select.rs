use super::msg_ctx::MessageProvider;
use super::yew_select_inner::YewSelectInner;
use super::msg_ctx::MessageContext;
use std::collections::HashMap;

use yew::prelude::*;

pub struct YewSelect {

}

pub enum YewSelectMsg {
}


#[derive(Clone, PartialEq, Properties)]
pub struct YewSelectProps {
    #[prop_or_default]
    pub children: Children,

    pub data:Vec<String>
}
impl Component for YewSelect {
    type Message = YewSelectMsg;
    type Properties = YewSelectProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MessageProvider>
                <YewSelectInner data={ctx.props().data.clone()}>
                    // {ctx.props().children.clone()}
                </YewSelectInner >
            </MessageProvider>
        }
    }
}
