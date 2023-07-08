use yew::prelude::*;

pub enum Msg {}

pub struct YewColorAlphaSlider {}

#[derive(Clone, PartialEq, Properties)]
pub struct YewColorAlphaSliderProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub vertical: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub on_change: Callback<(f64, f64)>,
}

impl Component for YewColorAlphaSlider {
    type Message = Msg;
    type Properties = YewColorAlphaSliderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}
