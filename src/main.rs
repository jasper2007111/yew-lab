use yew::prelude::*;
use yew_router::prelude::*;

mod pages;
mod route;
mod components;
mod bindings;

use pages::call_js_page::CallJsPage;
use pages::contexts::contexts_page::ContextsPage;
use pages::counter_page::CounterPage;
use pages::fetch_page::FetchPage;
use pages::home_page::HomePage;
use pages::img_page::ImgPage;
use crate::route::Route;

use gloo_console::log;

use std::rc::Rc;
use yewdux::prelude::*;

fn switch(routes: Route) -> Html {
    log!("路由！！！");
    match routes {
        Route::HomePage => html! { <HomePage/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::ImgPage => html! { <ImgPage/> },
        Route::FetchPage => html! { <FetchPage/> },
        Route::CounterPage => html! { <CounterPage/>},
        Route::CallJsPage => html!(<CallJsPage/>),
        Route::ContextsPage => html!(<ContextsPage/>)
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <>
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
