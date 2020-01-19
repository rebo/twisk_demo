use seed::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Popper;

    #[wasm_bindgen(constructor)]
    pub fn new(
        reference: web_sys::HtmlElement,
        popper: web_sys::HtmlElement,
        options: JsValue,
    ) -> Popper;

    #[wasm_bindgen(method)]
    pub fn scheduleUpdate(this: &Popper);
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Options {
    pub placement: String,
}
