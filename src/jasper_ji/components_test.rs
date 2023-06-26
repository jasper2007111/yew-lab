use gloo_console::log;
use yew::prelude::*;
use super::yew_button::YewButton;

pub enum Msg {
    BtnClick
}

pub struct ComponentsTest {
}

impl Component for ComponentsTest {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BtnClick=>{
                log!("按钮点击");
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(move|_e:MouseEvent|{
            Msg::BtnClick
        });
        html! {
            <div class="center-container">
                <h1>{ "组件测试" }</h1>
                <YewButton style="primary" title="按钮测试" on_clicked={on_clicked.clone()} />
                <br/>
                <YewButton style="primary" title="按钮测试" loading={true}  on_clicked={on_clicked.clone()} />
            </div>
        }
    }
}
