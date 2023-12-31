use super::msg_ctx::MessageProvider;
use super::producer::Producer;
use super::struct_component_subscriber::StructComponentSubscriber;
use super::subscriber::Subscriber;
use super::contexts_components::ContextsComponents;
use super::struct_component_producer::SCProducer;
use yew::prelude::*;

#[function_component]
pub fn ContextsPage() -> Html {
    html! {
        <ContextsComponents>
            <Producer />
            <SCProducer />
        </ContextsComponents>
    }
}