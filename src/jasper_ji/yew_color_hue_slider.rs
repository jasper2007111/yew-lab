use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlElement};
use yew::prelude::*;

use super::yew_button::YewButton;

pub enum Msg {
    None,
    OnBarClick(MouseEvent),
}
pub struct YewColorHueSlider {
    thumb_top: f64,
    thumb_left: f64,
    show_panel_color: bool,
    props: YewColorHueSliderProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewColorHueSliderProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or(true)]
    pub vertical: bool,

    #[prop_or_default]
    pub hue:f64,

    #[prop_or_default]
    pub on_change:Callback<f64>
}

impl Component for YewColorHueSlider {
    type Message = Msg;
    type Properties = YewColorHueSliderProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            thumb_top: 0.0,
            thumb_left: 0.0,
            show_panel_color: false,
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::OnBarClick(e) => {
                let target: HtmlElement = e.target_unchecked_into();
                let rect = target.get_bounding_client_rect();
                let document = web_sys::window().unwrap().document().unwrap();
                let thumb = document
                    .query_selector("#thumb")
                    .unwrap()
                    .unwrap()
                    .dyn_ref::<HtmlElement>()
                    .unwrap()
                    .clone();

                let hue;

                if !self.props.vertical {
                    let mut left = e.client_x() as f64 - rect.left();
                    left = js_sys::Math::min(left, rect.width() - thumb.offset_width() as f64 /2.0);
                    left = js_sys::Math::max(thumb.offset_width() as f64 / 2.0, left);

                    hue = js_sys::Math::round((left - thumb.offset_width() as f64 / 2.0) / (rect.width() - thumb.offset_width() as f64) * 360.0);
                } else {
                    let mut top = e.client_y() as f64 - rect.top();
                    top =
                        js_sys::Math::min(top, rect.height() - thumb.offset_height() as f64 / 2.0);
                    top = js_sys::Math::max(thumb.offset_height() as f64 / 2.0, top);

                    hue = js_sys::Math::round(
                        (top - thumb.offset_height() as f64 / 2.0)
                            / (rect.height() - thumb.offset_height() as f64)
                            * 360.0,
                    );
                }
                self.thumb_left = if self.props.vertical {
                    0.0
                } else {
                    js_sys::Math::round(hue * (target.offset_width() as f64 - thumb.offset_width() as f64/2.0)/360.0)
                };

                self.thumb_top = if !self.props.vertical {
                    0.0
                } else {
                    js_sys::Math::round(hue * (target.offset_height() as f64 - thumb.offset_height() as f64/2.0)/360.0)
                };

                self.props.on_change.emit(hue);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="el-color-hue-slider is-vertical">
                <div class="el-color-hue-slider__bar" onclick={ctx.link().callback(|e|{
                    Msg::OnBarClick(e)
                })}></div>
                <div id="thumb" class="el-color-hue-slider__thumb" style={format!("left: {}px; top: {}px", self.thumb_left, self.thumb_top)}>
                </div>
            </div>
        }
    }
}

impl YewColorHueSlider {
    
}
