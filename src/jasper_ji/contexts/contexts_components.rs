use super::msg_ctx::MessageProvider;
use super::producer::Producer;
use super::struct_component_subscriber::StructComponentSubscriber;
use super::subscriber::Subscriber;

use super::msg_ctx::MessageContext;


use yew::prelude::*;

pub struct ContextsComponents {

}

pub enum ContextsComponentsMsg {
}


#[derive(Clone, PartialEq, Properties)]
pub struct ContextsComponentsProps {
    pub children: Children,
}
impl Component for ContextsComponents {
    type Message = ContextsComponentsMsg;
    type Properties = ContextsComponentsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <MessageProvider>
                // <Producer />
                <Subscriber />
                <StructComponentSubscriber >
                {ctx.props().children.clone()}
                </StructComponentSubscriber >
            </MessageProvider>
        }
    }
}
