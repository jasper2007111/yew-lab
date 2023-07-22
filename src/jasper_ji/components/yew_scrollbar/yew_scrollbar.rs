use yew::prelude::*;

pub struct YewScrollbar;

pub enum YewScrollbarMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewScrollbarProps {
    #[prop_or_default]
    pub children: Children
}

impl Component for YewScrollbar {
    type Message = YewScrollbarMsg;
    type Properties = YewScrollbarProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="el-scrollbar">
                <div class="el-select-dropdown__wrap el-scrollbar__wrap el-scrollbar__wrap--hidden-default">
                    <ul class="el-scrollbar__view el-select-dropdown__list">
                        {ctx.props().children.clone()}
                    </ul>
                </div>
            </div>
        }
    }
}