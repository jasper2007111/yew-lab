use web_sys::console;
use yew::prelude::*;

use yew_router::prelude::*;

use super::route::Route;

pub enum Msg {
    SetEdit(usize),
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub name: String,
    pub route: Route,
}

pub struct Home {
    menus: Vec<Menu>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            menus: vec![
                Menu {
                    name: String::from("图片测试"),
                    route: Route::ImgTest,
                },
                Menu {
                    name: String::from("Fetch测试"),
                    route: Route::FetchTest,
                },
                Menu {
                    name: String::from("Counter测试"),
                    route: Route::CounterTest,
                },
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        for (i, menu) in self.menus.clone().into_iter().enumerate() {
            list.push(html! {
                <div onclick={ctx.link().callback(move|_|Msg::SetEdit(i))} >{menu.name}</div>
            })
        }
        html! {
            <div>
                <h1>{ "主页" }</h1>
                {list}
            </div>
        }
    }
}
