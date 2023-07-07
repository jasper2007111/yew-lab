
use yew::prelude::*;

pub enum Msg {}
pub struct YewButton {
    props:YewButtonProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub style: String,
    pub title: AttrValue,
    pub on_clicked: Callback<MouseEvent>,
    #[prop_or_default]
    pub loading:bool,

    #[prop_or_default]
    pub plain:bool,
    
    // medium / small / mini
    #[prop_or_default]
    pub size:String
}

impl Component for YewButton {
    type Message = Msg;
    type Properties = YewButtonProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props:ctx.props().clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = ctx.props().title.clone();
        let disabled = ctx.props().disabled.clone();
        let style = ctx.props().style.clone();
        let loading = ctx.props().loading.clone();
        
        let onclick = ctx.props().on_clicked.reform(move |event: MouseEvent| {
            event.stop_propagation();
            event.clone()
        });

    
        let mut classes = Vec::new();
        classes.push(String::from("el-button"));

        if !style.is_empty() {
            let ss = format!("el-button--{}", style);
            classes.push(ss);
        }
        if disabled {
            classes.push(String::from("is-disabled"));
        }

        if self.props.plain {
            classes.push(String::from("is-plain"));
        }

        // TODO 需要对字符串进行检查
        if !self.props.size.is_empty() {
            classes.push(format!("el-button--{}", self.props.size));
        }

        html! {
            <button class={classes!(classes.clone())} {onclick} disabled={disabled.clone()} >
            {title.clone()}
            if loading {
                <i class="el-icon-loading"></i>
            }
            </button>
        }
    }
}
