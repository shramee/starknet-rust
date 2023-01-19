mod starknet_bindings;
use starknet_bindings::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn logAny(s: bool);
}

#[wasm_bindgen]
pub fn greet() {
    let s = String::from("");
    let st = " yo ";
		st.trim
    st[i..j];
    let _ = s.chars().enumerate();
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

#[wasm_bindgen(start)]
pub async fn run() {
    log("Connecting...");
    log_conn_status();
    connect().await;
    log_conn_status();
}

fn log_conn_status() {
    if STARKNET.is_connected() {
        log("Connected!");
    } else {
        log("Not connected");
    }
}

pub async fn connect() {
    STARKNET.enable().await;
}
