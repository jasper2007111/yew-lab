use std::io::Cursor;
use std::io::Read;

use serde::de::IntoDeserializer;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;
use yew::Callback;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;

use yew_router::prelude::*;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub enum Msg {
    GetUser
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: String
}

use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
async fn run(repo: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://127.0.0.1:8092");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}

pub struct FetchTest {}


impl Component for FetchTest {
    type Message = Msg;
    type Properties = ();

    

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(async {
           match run(String::from("repo")).await {
               Ok(data)=>{
                let note: User = serde_wasm_bindgen::from_value(data).expect("msg");
                console::log_1(&JsValue::from(note.name));
                Msg::GetUser
               },
               Err(_)=>Msg::GetUser
           }
        });
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        
        match msg {
            Msg::GetUser=>{
                true
            }
        }
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

        let scalex = 3.0 / imgx as f32;
        let scaley = 3.0 / imgy as f32;

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

        console::log_1(&JsValue::from(uint8arr.length()));

        let array = js_sys::Array::new();
        array.push(&uint8arr.buffer());

        let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("image/png"),
        )
        .unwrap();

        console::log_1(&blob.size().into());

        let image_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        console::log_1(&JsValue::from(image_url.clone()));
        html! {
            <div>
                <h1>{ "主页" }</h1>
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
