use gloo_console::log;
use yew::prelude::*;

use yew_router::prelude::*;

use crate::route::Route;

pub enum Msg {
    None,
    SetEdit(usize),
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub name: String,
    pub route: Route,
}

pub struct HomePage {
    menus: Vec<Menu>,
}

impl Component for HomePage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            menus: vec![
                Menu {
                    name: String::from("图片测试"),
                    route: Route::ImgPage,
                },
                Menu {
                    name: String::from("Fetch测试"),
                    route: Route::FetchPage,
                },
                Menu {
                    name: String::from("Counter测试"),
                    route: Route::CounterPage,
                },
                Menu {
                    name: String::from("调用JS测试"),
                    route: Route::CallJsPage
                },
                Menu {
                    name: String::from("Contexts测试"),
                    route: Route::ContextsPage
                },
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None=>{
                log!("dddd");
                false
            },
            Msg::SetEdit(id) => {
                let history1 = ctx.link().navigator().unwrap();
                let menu = self.menus.get(id).unwrap();
                history1.push(&menu.route);
                false
            }
        }
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut list = vec![];
        let on_clicked = ctx.link().callback(move|event:MouseEvent|{
            Msg::None
        });
        for (i, menu) in self.menus.clone().into_iter().enumerate() {
            list.push(html! {
                <div class={"row"} onclick={ctx.link().callback(move|_|Msg::SetEdit(i))} >
                {menu.name} 
                </div>
            })
        }
        html! {
            // <div class="center-container">
            //     <div>{"主页"}</div>
            //     <div>{list}</div>
            // </div>
            <>{list}</>
        }
    }
}
