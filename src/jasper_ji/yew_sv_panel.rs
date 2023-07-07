use yew::prelude::*;

use super::yew_button::YewButton;
use web_sys::HtmlElement;
use wasm_bindgen::JsCast;

/**
 * 
 * TODO 拖拽选择功能暂未实现，实现有问题
 */
pub enum Msg {
    None,
    OnClick(MouseEvent),
    OnMouseDown(MouseEvent),
    OnMouseMove(MouseEvent),
    OnMouseUp(MouseEvent),
}
pub struct YewSvPanel {
    is_dragging:bool,
    cursor_top: f64,
    cursor_left: f64,
    background: String,
    props: YewSvPanelProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct YewSvPanelProps {
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub hue: f64,

    #[prop_or_default]
    pub on_change:Callback<(f64, f64)>
}

impl Component for YewSvPanel {
    type Message = Msg;
    type Properties = YewSvPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_dragging: false,
            cursor_top: 0.0,
            cursor_left: 0.0,
            background: format!("hsl(0, 100%, 50%)"),
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::OnClick(e) =>{
                self.handle_drag(e);
                true
            }
            Msg::OnMouseDown(e) => {
                if self.is_dragging {
                    return false;
                }
                self.is_dragging = true;
                self.handle_drag(e);
                true
            },
            Msg::OnMouseMove(e) => {
                self.handle_drag(e);
                true
            },
            Msg::OnMouseUp(e) => {
                self.is_dragging = false;
                self.handle_drag(e);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hue = ctx.props().hue;
        let background = format!("hsl({}, 100%, 50%)", hue);

        html! {
            <div class="el-color-svpanel" style={format!("background-color: {};", background)} 
            onclick={ctx.link().callback(|e|{
                Msg::OnClick(e)
            })} onmousedown={ctx.link().callback(|e|{
                Msg::None
            })} onmousemove={ctx.link().callback(|e|{
                Msg::None
            })} onmouseup={ctx.link().callback(|e|{
                Msg::None
            })}>
                <div class="el-color-svpanel__white"></div>
                <div class="el-color-svpanel__black"></div>
                <div class="el-color-svpanel__cursor" style={format!("top: {}px; left: {}px", self.cursor_top, self.cursor_left)}>
                    <div></div>
                </div>
            </div>
        }
    }
}

impl YewSvPanel {
    pub fn handle_drag(& mut self, e: MouseEvent) {
        let target: HtmlElement = e.target_unchecked_into();
        let rect = target.get_bounding_client_rect();

        let mut left = e.client_x() as f64 -rect.left();
        let mut top = e.client_y() as f64 - rect.top();

        left = js_sys::Math::max(0.0, left);
        left = js_sys::Math::min(left, rect.width() as f64);

        top = js_sys::Math::max(0.0, top);
        top = js_sys::Math::min(top, rect.height() as f64);

        self.cursor_left = left;
        self.cursor_top = top;

        let saturation = left/rect.width()*100.0;
        let value = 100.0-top/rect.height()*100.0;
        self.props.on_change.emit((saturation, value));
    }
}
