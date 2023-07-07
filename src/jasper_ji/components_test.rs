use super::yew_button::YewButton;
use super::yew_rate::YewRate;
use super::yew_color_picker::YewColorPicker;
use gloo_console::log;
use yew::prelude::*;

pub enum Msg {
    BtnClick,
    OnRateValueChanged(f64)
}

pub struct ComponentsTest {}

impl Component for ComponentsTest {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::BtnClick => {
                log!("按钮点击");
                false
            },
            Msg::OnRateValueChanged(v) =>{
                log!("v: ", v);
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_clicked = ctx.link().callback(move |_e: MouseEvent| Msg::BtnClick);
        html! {
            <div style="margin-left: 100px;">
                <h1>{ "组件测试" }</h1>
                <YewButton style="primary" title="按钮测试" on_clicked={on_clicked.clone()} />
                <br/>
                <br/>
                <YewRate value={0.0} on_change={ctx.link().callback(|v| {
                    Msg::OnRateValueChanged(v)
                })} show_text={true} allow_half={true}/>
                <br/>
                <YewColorPicker />
            </div>
        }
    }
}
