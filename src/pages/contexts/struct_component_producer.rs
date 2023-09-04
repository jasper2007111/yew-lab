use yew::prelude::*;

use super::msg_ctx::MessageContext;

pub struct SCProducer {
    message: MessageContext
}

pub enum ProducerMsg {
    OnClick,
}

impl Component for SCProducer {
    type Message = ProducerMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (message, _) = ctx
            .link()
            .context(Callback::default())
            .expect("No Message Context Provided");

        Self {
            message
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ProducerMsg::OnClick => {
                self.message.dispatch("SCProducer Message Received.".to_string());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <button onclick={ctx.link().callback(|_| ProducerMsg::OnClick)}>
                {"Struct Component Producer PRESS ME"}
            </button>
        }
    }
}
