use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

async fn run_async() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(start)]
pub fn run() {
    spawn_local(async {
        run_async().await.expect("Failed to run:");
    });
}