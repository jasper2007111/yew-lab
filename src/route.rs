use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/imgPage")]
    ImgPage,
    #[at("/fetchPage")]
    FetchPage,
    #[at("/counterPage")]
    CounterPage,
    #[at("/callJsPage")]
    CallJsPage,
    #[at("/contextPage")]
    ContextsPage,
}
