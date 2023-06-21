use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/imgTest")]
    ImgTest,
    #[at("/fetchTest")]
    FetchTest
}
