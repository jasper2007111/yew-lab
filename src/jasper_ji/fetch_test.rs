use wasm_bindgen::JsValue;
use yew::prelude::*;
use wasm_bindgen::JsCast;

use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub enum Msg {
    Loading,
    GetUserOk(User),
    GetUserErr(String),
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String,
}

async fn run(repo: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://127.0.0.1:8092");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}

pub struct FetchTest {
    user_msg: Msg
}

impl Component for FetchTest {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
            match run(String::from("repo")).await {
                Ok(data) => {
                    let user: User = serde_wasm_bindgen::from_value(data).expect("msg");
                    Msg::GetUserOk(user)
                }
                Err(_) => Msg::GetUserErr(String::from("Error")),
            }
        });
        Self {
            user_msg:Msg::Loading
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loading=>false,
            Msg::GetUserOk(user) => {
                self.user_msg = Msg::GetUserOk(user);
                true
            },
            Msg::GetUserErr(_)=>false,
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let content;
        match &self.user_msg  {
            Msg::Loading => {
                content = html!(
                    <div>{"loading"}</div>
                );
            },
            Msg::GetUserErr(_)=>{
                content = html!(
                    <div>{"加载错误"}</div>
                );
            },
            Msg::GetUserOk(user)=>{
                content = html!(
                    <div>{"user.name: "}{&user.name}</div>
                );
            }
        }

        html!(
            <div>
                <h1>{"Fetch测试"}</h1>
                {content}
            </div>
        )
    }
}
