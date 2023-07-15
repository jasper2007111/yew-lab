use yew::prelude::*;

use super::components::yew_button::YewButton;
use super::components::yew_input::YewInput;

pub struct Login;

pub enum LoginMsg {}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="display: flex; width: 100%; height: 100vh; flex-direction: row">
                <div style="width: 300px; align-items: center; justify-content: center; margin:auto;">
                <div><span>{"用户名"}</span><YewInput/></div>
                <br/>
                <div><span>{"密码"}</span><YewInput/></div>
                <br/>
                <div><YewButton button_type="primary" title="登录" width={"100%"}/></div>
                </div>
            </div>
        }
    }
}
