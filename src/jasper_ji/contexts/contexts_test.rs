use super::msg_ctx::MessageProvider;
use super::producer::Producer;
use super::struct_component_subscriber::StructComponentSubscriber;
use super::subscriber::Subscriber;
use super::contexts_components::ContextsComponents;
use yew::prelude::*;

#[function_component]
pub fn ContextsTest() -> Html {
    html! {
        <ContextsComponents>
            <Producer />
        </ContextsComponents>
    }
}