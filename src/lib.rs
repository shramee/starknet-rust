mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logAny(s: JsValue);

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn id() -> Result<String, JsValue>;
    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn name() -> Result<String, JsValue>;
    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn version() -> Result<String, JsValue>;
    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn icon() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn selectedAddress() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn chainId() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn enable() -> Result<String, JsValue>;
}

#[wasm_bindgen]
pub fn greet() {}

#[wasm_bindgen(start)]
pub fn run() {
    let msg = name();
    match msg {
        Ok(val) => log(&val),
        Err(_err) => logAny(_err),
    }
    let msg = version();
    match msg {
        Ok(val) => log(&val),
        Err(_err) => logAny(_err),
    }
}
