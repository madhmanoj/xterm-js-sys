use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[wasm_bindgen(module = "@xterm/xterm")]
extern "C" {
    #[derive(Clone)]
    pub type Terminal;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Terminal;

    #[wasm_bindgen(method)]
    pub fn open(this: &Terminal, parent: &HtmlDivElement);

    #[wasm_bindgen(method)]
    pub fn write(this: &Terminal, data: &str);
}

// FitAddon bindings
#[wasm_bindgen(module = "@xterm/addon-fit")]
extern "C" {
    #[derive(Clone)]
    pub type FitAddon;

    #[wasm_bindgen(constructor)]
    pub fn new() -> FitAddon;

    #[wasm_bindgen(method)]
    pub fn fit(this: &FitAddon);

    #[wasm_bindgen(method)]
    pub fn activate(this: &FitAddon, terminal: &Terminal);
}