use yew::prelude::*;
use yew_router::prelude::*;

mod jasper_ji;

use jasper_ji::home::Home;
use jasper_ji::img_test::ImgTest;
use jasper_ji::fetch_test::FetchTest;
use jasper_ji::counter_test::CounterTest;
use jasper_ji::components_test::ComponentsTest;
use jasper_ji::route::Route;
use jasper_ji::login::Login;
use jasper_ji::call_js_test::CallJsTest;


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::ImgTest => html! { <ImgTest/> },
        Route::FetchTest => html! { <FetchTest/> },
        Route::CounterTest =>html !{ <CounterTest/>},
        Route::ComponentsTest => html!{<ComponentsTest/>},
        Route::Login => html!(<Login/>),
        Route::CallJsTest=> html!(<CallJsTest/>)
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
