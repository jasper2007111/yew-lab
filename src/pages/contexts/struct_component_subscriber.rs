use yew::prelude::*;

use super::msg_ctx::MessageContext;

pub enum Msg {
    MessageContextUpdated(MessageContext),
}

pub struct StructComponentSubscriber {
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
}


#[derive(Clone, PartialEq, Properties)]
pub struct StructComponentSubscriberProps {
    pub children: Children,
}

impl Component for StructComponentSubscriber {
    type Message = Msg;
    type Properties = StructComponentSubscriberProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::MessageContextUpdated))
            .expect("No Message Context Provided");

        Self {
            message,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MessageContextUpdated(message) => {
                self.message = message;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            <h1>{ self.message.inner.to_string() }</h1>
            {                {ctx.props().children.clone()}
}
            </>
        }
    }
}