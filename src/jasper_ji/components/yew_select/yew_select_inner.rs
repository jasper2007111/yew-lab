use yew::{prelude::*, virtual_dom::VNode};

use super::msg_ctx::MessageContext;
use super::yew_select_menu::YewSelectMenu;
use super::yew_option::YewOption;
use crate::jasper_ji::components::yew_input::YewInput;
use std::collections::HashMap;


use gloo_console::log;

pub struct YewSelectInner {
    message: MessageContext,
    _context_listener: ContextHandle<MessageContext>,
    input_ref: NodeRef,
    visible: bool,
    props: YewSelectInnerProps,
}

pub enum YewSelectInnerMsg {
    MessageContextUpdated(MessageContext),
    OnFocus,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewSelectInnerProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub children: Children,

    pub data:Vec<String>,

    #[prop_or_default]
    pub on_change: Callback<String>
}

impl Component for YewSelectInner {
    type Message = YewSelectInnerMsg;
    type Properties = YewSelectInnerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (message, context_listener) = ctx
            .link()
            .context(
                ctx.link()
                    .callback(YewSelectInnerMsg::MessageContextUpdated),
            )
            .expect("No Message Context Provided");

        Self {
            visible: false,
            input_ref: NodeRef::default(),
            props: ctx.props().clone(),
            message,
            _context_listener: context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            YewSelectInnerMsg::MessageContextUpdated(message) => {
                self.message = message;
                self.visible = false;
                self.props.on_change.emit(self.message.inner.clone());
                true
            }
            YewSelectInnerMsg::OnFocus => {
                self.visible = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut menus = vec![];
        for value in ctx.props().data.clone().into_iter() {
            let selected = if self.message.inner == value {
                true
            } else {
                false
            };
            
            menus.push(html!(<YewOption label={value.clone()} value = {value.clone()} {selected}/>));
        }

        let mut suffix_classes = vec!["el-select__caret".to_string(), "el-input__icon".to_string(), "el-icon-arrow-up".to_string()];
        if self.visible {
            suffix_classes.push("is-reverse".to_string());
        } 

        html! {
            <div
                class={classes!(self.get_root_classes())}
            >
            <YewInput value={String::from(self.message.inner.clone())} on_focus={ctx.link().callback(|_|{
                YewSelectInnerMsg::OnFocus
            })}>
              <div slot="suffix">
              <i class={classes!(suffix_classes)}></i>
              </div>
            </YewInput>
            if self.visible {
                <YewSelectMenu>
                {menus}
                </YewSelectMenu>
            }
            </div>
        }
    }
}

impl YewSelectInner {
    fn get_root_classes(&self) -> Vec<String> {
        let mut vec = vec!["el-select".to_string()];
        if let Some(c) = self.get_select_size() {
            vec.push(c);
        }
        vec
    }
    fn get_select_size(&self) -> Option<String> {
        if !self.props.size.is_empty() {
            return Some(self.props.size.clone());
        }
        return None;
    }

    fn get_multiple_node(&self) -> VNode {
        html! {
            <></>
        }
    }
}
