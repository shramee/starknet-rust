use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (js_name = starknet)]
    pub static STARKNET: StarknetObj;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub type StarknetObj;

    # [wasm_bindgen (structural , method , getter , js_namespace = starknet , js_name = isConnected)]
    pub fn is_connected(this: &StarknetObj) -> bool;

    #[wasm_bindgen(structural , method , js_namespace = starknet)]
    pub async fn enable(this: &StarknetObj) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logAny(s: bool);
}

#[wasm_bindgen]
pub fn greet() {
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

#[wasm_bindgen(start)]
pub async fn run() {
    log("Connecting...");
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
    connect().await;
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

pub async fn connect() {
    STARKNET.enable().await;
}
