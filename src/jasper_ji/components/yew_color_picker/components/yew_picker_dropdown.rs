use gloo_console::log;
use yew::prelude::*;

use crate::jasper_ji::components::yew_button::YewButton;
use crate::jasper_ji::utils::yew_color::YewColor;

use super::yew_color_hue_slider::YewColorHueSlider;
use super::yew_sv_panel::YewSvPanel;

pub enum Msg {
    None,
    OnHandleTrigger(MouseEvent),
    OnHueChanged(f64),
    OnSvChanded((f64, f64)),
    OnConfirmValue,
    OnClearValue,
}
pub struct YewPickerDropdown {
    hue: f64,
    saturation: f64,
    value: f64,
    show_panel_color: bool,
    props: YewPickerDropdownProps,

    color_hex: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewPickerDropdownProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub show_alpha: bool,

    #[prop_or_default]
    pub on_change: Callback<String>,

    #[prop_or_default]
    pub on_confirm_value: Callback<String>,

    #[prop_or_default]
    pub on_clear_value: Callback<String>,

    #[prop_or_default]
    pub custom_input: String,
}

impl Component for YewPickerDropdown {
    type Message = Msg;
    type Properties = YewPickerDropdownProps;

    fn create(ctx: &Context<Self>) -> Self {
        if !ctx.props().value.is_empty() {
            let rgb_result = csscolorparser::parse(ctx.props().value.as_str());
            if rgb_result.is_ok() {
                let color = rgb_result.unwrap();
                let hsv = color.to_hsva();
                let h = hsv.0;
                let s: f64 = hsv.1;
                let v = hsv.2;

                let temp_saturation = js_sys::Math::floor(s * 100.0);
                let temp_value = js_sys::Math::floor(v * 100.0);

                return Self {
                    color_hex: color.to_hex_string(),
                    hue: h,
                    saturation: temp_saturation,
                    value: temp_value,
                    show_panel_color: false,
                    props: ctx.props().clone(),
                };
            }
        }
        Self {
            color_hex: String::default(),
            hue: 0.0,
            saturation: 0.0,
            value: 0.0,
            show_panel_color: false,
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::OnHandleTrigger(e) => false,
            Msg::OnHueChanged(v) => {
                self.hue = v;
                self.update_color();
                true
            }
            Msg::OnSvChanded((s, v)) => {
                // log!(format!("h: {}, s: {}, v:{}", self.hue, s, v));
                self.saturation = s;
                self.value = v;
                self.update_color();
                true
            }
            Msg::OnClearValue => {
                self.props.on_clear_value.emit("clear".to_string());
                false
            }
            Msg::OnConfirmValue => {
                self.props.on_confirm_value.emit("confirm".to_string());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let custom_input;
        if !self.color_hex.is_empty() {
            custom_input = self.color_hex.clone();
        } else {
            custom_input = self.props.custom_input.clone();
        }
        let mut classes = vec!["el-color-picker".to_string()];
        if self.get_color_disabled() {
            classes.push("is-disabled".to_string());
        }
        html! {
            <div class="el-color-dropdown el-color-picker__panel">
                <div class="el-color-dropdown__main-wrapper">
                    // <hue-slider ref="hue" :color="color" vertical style="float: right;"></hue-slider>
                    <div style="float: right;"><YewColorHueSlider hue={self.hue} on_change={ctx.link().callback(|v|{
                        Msg::OnHueChanged(v)
                    })}/></div>
                    <YewSvPanel hue={self.hue} saturation={self.saturation} value={self.value} on_change={ctx.link().callback(|e|{
                        Msg::OnSvChanded(e)
                    })}/>
                </div>
                // <alpha-slider v-if="showAlpha" ref="alpha" :color="color"></alpha-slider>
                // <predefine v-if="predefine" :color="color" :colors="predefine"></predefine>
                <div class="el-color-dropdown__btns">
                    <span class="el-color-dropdown__value">
                        // TODO 暂未实现input组件，使用文本代替
                        // <el-input
                        // v-model="customInput"
                        // @keyup.native.enter="handleConfirm"
                        // @blur="handleConfirm"
                        // :validate-event="false"
                        // size="mini">
                        // </el-input>
                        {custom_input}
                    </span>
                    <YewButton style="text" size={"mini"} title={"清空"} on_clicked={ctx.link().callback(|_|{
                        Msg::OnClearValue
                    })}/>
                    <YewButton size={"mini"} plain={true} title={"确定"} on_clicked={ctx.link().callback(|_|{
                        Msg::OnConfirmValue
                    })}/>
                </div>
            </div>
        }
    }
}

impl YewPickerDropdown {
    pub fn get_color_disabled(&self) -> bool {
        self.props.disabled
    }

    pub fn get_show_alpha_class(&self) -> String {
        if self.props.show_alpha {
            return "is-alpha".to_string();
        }
        "".to_string()
    }

    pub fn get_displayed_color(&self) -> String {
        "#fff".to_string()
    }

    pub fn update_color(&mut self) {
        let rgb = YewColor::hsv2rgb(self.hue, self.saturation, self.value);
        // log!(format!("r: {}, g: {}, b:{}", rgb.0, rgb.1, rgb.2));
        self.props.custom_input = YewColor::rgb2hex(rgb.0, rgb.1, rgb.2);

        self.props.on_change.emit(self.props.custom_input.clone());
        self.color_hex = String::default();
        // log!(self.props.custom_input.clone());
    }
}
