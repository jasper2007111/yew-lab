use yew::prelude::*;
use yew_router::prelude::*;

use crate::bindings::{init_editor, get_text};
use gloo_console::log;

use serde::{Deserialize, Serialize};

pub struct CallJsPage {
    title: String,
    content: String
}

pub enum CallJsTestMsg {
    None,
    BtnClick,
    OnInputChanged(String),
    OnCreateSuccess,
}

impl Component for CallJsPage {
    type Message = CallJsTestMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            title: String::default(),
            content: String::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CallJsTestMsg::None =>{
                false
            },
            CallJsTestMsg::OnCreateSuccess => {
                let history1 = ctx.link().navigator().unwrap();
                history1.back();
                false
            },
            CallJsTestMsg::BtnClick => {
                let ttt = get_text();
                log!("按钮点击:", ttt);
                false
            },
            CallJsTestMsg::OnInputChanged(s)=>{
                self.title = s;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>  
                <div id="editor—wrapper" style="border: 1px solid #ccc; z-index: 100;">
                    <div id="toolbar-container" style="border-bottom: 1px solid #ccc;"></div>
                    <div id="editor-container" style="height: 500px;"></div>
                </div>
                <div onclick={ctx.link().callback(|_|{
                    CallJsTestMsg::BtnClick
                })}>
                    {"提交"}
                </div>
            </>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let d = init_editor();
        }
    }
}
