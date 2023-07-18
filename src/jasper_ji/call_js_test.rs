use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yew::suspense::{use_future, SuspensionResult};
use yew::use_memo;

use crate::jasper_ji::bindings::{hello, get_text};
use super::components::yew_button::YewButton;
use gloo_console::log;


// #[function_component]
// fn CallJsTest() -> Html {
//     let msg = use_memo((), |_| hello());
//     html! {
//         <>
//             <h2>{"Important"}</h2>
//             <p>{msg}</p>
//         </>
//     }
// }

// #[derive(PartialEq, Eq, Debug)]
// pub struct CallJsTest;

// #[derive(Clone, Debug, PartialEq, Eq, Properties)]
// pub struct CallJsTestProps {
//     #[prop_or_default]
//     pub step: usize,
// }

// #[function_component]
// pub fn CallJsTest(props: &CallJsTestProps) -> Html {
//     // Will only get recalculated if `props.step` value changes
//     // let message = use_memo(
//     //     |step| format!("{}. Do Some Expensive Calculation", step),
//     //     props.step,
//     // );

//     // html! {
//     //     <div>
//     //         <span>{ (*message).clone() }</span>
//     //     </div>
//     // }

//     // let msg = use_memo((), |_| hello());
//     let d = hello();
//     html! {
//             <>
//                 <h2>{"Important"}</h2>
//                 <p>{d}</p>
//                 <div id="editor—wrapper">
//       <div id="toolbar-container"></div>
//       <div id="editor-container"></div>
//     </div>
//             </>
//         }
// }

pub struct CallJsTest;

pub enum CallJsTestMsg {
    BtnClick
}

impl Component for CallJsTest {
    type Message = CallJsTestMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CallJsTestMsg::BtnClick => {
                let ttt = get_text();
                log!("按钮点击:", ttt);
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div id="editor—wrapper" style="border: 1px solid #ccc; z-index: 100;">
                    <div id="toolbar-container" style="border-bottom: 1px solid #ccc;"></div>
                    <div id="editor-container" style="height: 500px;"></div>
                </div>
                <YewButton button_type="primary" title="按钮测试" on_clicked={ctx.link().callback(|_|{
                    CallJsTestMsg::BtnClick
                })} />
            </>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let d = hello();
        }
    }
}
