use std::io::Cursor;

use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;
use yew::Callback;

use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;

use yew_router::prelude::*;

pub enum Msg {}

pub struct ImgTest {}

impl Component for ImgTest {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let history = ctx.link().navigator().unwrap();
        let onclick = Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let _content_element = document
                .get_element_by_id("content")
                .unwrap()
                .dyn_into::<HtmlTextAreaElement>()
                .unwrap();

            history.back();
        });

        let imgx = 800;
        let imgy = 800;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, 0, b]);
        }

        let mut bytes: Vec<u8> = Vec::new();
        imgbuf
            .write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
            .unwrap();

        let uint8arr: js_sys::Uint8Array =
            js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&bytes) }.into());

        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());

        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("image/png"),
        )
        .unwrap();

        let image_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        html! {
            <div>
                <h1>{ "图片测试" }</h1>
                <img src={image_url} width="200" height="200"/>
                <img src="./public/test.png" width="200" height="200"/>
                <br/>
                <textarea id="content" placeholder="输入内容"></textarea>
                <div>
                <button {onclick}>{ "提交" }</button>
                </div>
            </div>
        }
    }
}
