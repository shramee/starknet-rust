mod utils;

use futures::executor::block_on;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
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

    #[wasm_bindgen(js_namespace = starknet)]
    pub static id: String;
    #[wasm_bindgen(js_namespace = starknet)]
    pub static name: String;
    #[wasm_bindgen(js_namespace = starknet)]
    pub static version: String;
    #[wasm_bindgen(js_namespace = starknet)]
    pub static icon: String;

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn selectedAddress() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = starknet, catch)]
    pub fn chainId() -> Result<String, JsValue>;

    #[wasm_bindgen(js_namespace = starknet,)]
    async fn enable() -> JsValue;

    #[wasm_bindgen(js_namespace = starknet, js_name=isPreauthorized, catch)]
    async fn is_preauthorised() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub fn greet() {}

#[wasm_bindgen(start)]
pub async fn run() {
    let preauth = is_preauthorised().await;
    let _status = enable().await;

    logAny(_status)
    // match preauth {
    //     Ok(val) => logAny(val),
    //     Err(e) => logAny(e),
    // }
}
