use yew::{prelude::*, virtual_dom::VNode};

pub struct YewSelectMenu {
    props: YewSelectMenuProps
}

pub enum YewSelectMenuMsg {
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewSelectMenuProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub size: String,

    #[prop_or_default]
    pub children: Children,

}

impl Component for YewSelectMenu {
    type Message = YewSelectMenuMsg;
    type Properties = YewSelectMenuProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="el-select-dropdown el-popper">
                {self.props.children.clone()}
            </div>
        }
    }
}