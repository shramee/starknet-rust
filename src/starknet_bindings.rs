use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (js_name = starknet)]
    pub static STARKNET: StarknetObj;

    # [wasm_bindgen (js_name = starknet_argentX)]
    pub static STARKNET_ARGENTX: StarknetObj;

    # [wasm_bindgen (js_name = starknet_braavos)]
    pub static STARKNET_BRAAVOS: StarknetObj;

    #[wasm_bindgen]
    #[derive(Debug, Clone)]
    pub type StarknetObj;

    # [wasm_bindgen (structural , method , getter , js_namespace = starknet , js_name = isConnected)]
    pub fn is_connected(this: &StarknetObj) -> bool;

    #[wasm_bindgen(structural , method , js_namespace = starknet)]
    pub async fn enable(this: &StarknetObj) -> JsValue;
}
