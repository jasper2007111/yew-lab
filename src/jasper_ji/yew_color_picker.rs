
use yew::prelude::*;

use super::yew_picker_dropdown::YewPickerDropdown;

pub enum Msg {
    OnHandleTrigger(MouseEvent)
}
pub struct YewColorPicker {
    show_picker:bool,
    show_panel_color: bool,
    props:YewColorPickerProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewColorPickerProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub show_alpha: bool,

    #[prop_or_default]
    pub on_clicked: Callback<MouseEvent>
}

impl Component for YewColorPicker {
    type Message = Msg;
    type Properties = YewColorPickerProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            show_picker:false,
            show_panel_color: false,
            props: ctx.props().clone()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnHandleTrigger(e)=>{
                if self.get_color_disabled() {
                    return false;
                }
                self.show_picker = !self.show_picker;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut classes = vec!["el-color-picker".to_string()];
        if self.get_color_disabled() {
            classes.push("is-disabled".to_string());
        }
        html! {
            <div class={classes!(classes)}>
                if self.get_color_disabled() {
                    <div class="el-color-picker__mask"></div>
                }
                <div class="el-color-picker__trigger" onclick={ctx.link().callback(move |e: MouseEvent| { 
                    Msg::OnHandleTrigger(e)
                })}>
                    <span class={format!("el-color-picker__color {}", self.get_show_alpha_class())}>
                        <span class="el-color-picker__color-inner" style={format!("background-color: {};", self.get_displayed_color())}>
                        </span>
                        if self.props.value.is_empty() || !self.show_panel_color {
                            <span class="el-color-picker__empty el-icon-close"></span>
                        }
                    </span>
                    if !self.props.value.is_empty() || self.show_panel_color {
                        <span class="el-color-picker__icon el-icon-arrow-down"></span>
                    }
                </div>
                if self.show_picker {
                    <YewPickerDropdown/>
                }
            </div>
        }
    }
}

impl YewColorPicker {
    pub fn get_color_disabled(&self)->bool {
        self.props.disabled
    }

    pub fn get_show_alpha_class(&self)->String {
        if self.props.show_alpha {
            return "is-alpha".to_string();
        }
        "".to_string()
    }

    pub fn get_displayed_color(&self) ->String {
        "#fff".to_string()
    }
}

