use std::collections::HashMap;
use std::io::Cursor;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use gloo::file::callbacks::FileReader;
use gloo::file::File;
use gloo_console::log;
use yew::prelude::*;
use yew::Callback;

use wasm_bindgen::JsCast;
use web_sys::FileList;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

use yew_router::prelude::*;

struct FileDetails {
    name: String,
    file_type: String,
    data: Vec<u8>,
}

pub enum Msg {
    None,
    Loaded(String, String, Vec<u8>),
    Files(Vec<File>),
}

pub struct ImgPage {
    readers: HashMap<String, FileReader>,
    files: Vec<FileDetails>,
}

impl Component for ImgPage {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::None => false,
            Msg::Loaded(file_name, file_type, data) => {
                self.files.push(FileDetails {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let file_type = file.raw_mime_type();

                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
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
            <div class="center-container">
                <h1>{ "图片测试" }</h1>
                <div>{"显示通过代码创建的图像"}</div>
                <img src={image_url.clone()} width="200" height="200" onload={ctx.link().callback(move |_: Event| {
                    let _ = web_sys::Url::revoke_object_url(&image_url.clone());
                    Msg::None
                })}/>
                <div>{"显示内置资源图片"}</div>
                <img src="./public/test.png" width="200" height="200"/>
                <br/>
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
                <input
                    id="file-upload"
                    type="file"
                    accept="image/*,video/*"
                    multiple={true}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        log!("input.files(): ", input.files());
                        Self::upload_files(input.files())
                    })}
                />
                <div>
                <button {onclick}>{ "提交" }</button>
                </div>
            </div>
        }
    }
}

impl ImgPage {
    fn view_file(file: &FileDetails) -> Html {
        // let uint8arr: js_sys::Uint8Array =
        //     js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&file.data) }.into());

        // let array = js_sys::Array::new();
        // array.push(&uint8arr.buffer());

        // let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        //     &array,
        //     web_sys::BlobPropertyBag::new().type_(&format!("image/{}", file.file_type)),
        // )
        // .unwrap();

        // let image_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        // <img src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} />
        // <img src={image_url} />
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ format!("{}", file.name) }</p>
                <div class="preview-media">
                    if file.file_type.contains("image") {
                        <img src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} />
                    } else if file.file_type.contains("video") {
                        <video controls={true}>
                            <source src={format!("data:{};base64,{}", file.file_type, STANDARD.encode(&file.data))} type={file.file_type.clone()}/>
                        </video>
                    }
                </div>
            </div>
        }
    }
    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result: Vec<File> = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}
