use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use gloo_storage::LocalStorage;
use gloo_storage::Storage;

use std::collections::HashMap;
use gloo_console::log;

static BASE_URL: &str = "http://127.0.0.1:3000/api";

pub async fn run_request(method: String, url: String, params: Option<HashMap<String, String>>, data: Option<JsValue>) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(&method);
    opts.mode(RequestMode::Cors);

    if let Some(d) = data {
        opts.body(Some(js_sys::JSON::stringify(&d).unwrap().as_ref()));
    }

    let mut url = format!("{}{}", BASE_URL, url);

    if let Some(p) = params {
        let mut param_vec = vec![];
        for (k, v) in p.into_iter() {
            param_vec.push(format!("{}={}", k, v));
        }
        // log!(param_vec.join("&"));
        url.push_str(&param_vec.join("&"));
    }

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json")?;
    request.headers().set("Content-Type", "application/json")?;

    let token:String =  LocalStorage::get("token").unwrap_or_default();
    if !token.is_empty() {
        request.headers().set("Authorization", &format!("Bearer {}", token))?;
    }

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

