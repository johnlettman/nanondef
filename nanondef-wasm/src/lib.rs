mod error;
mod message;
mod tag;

pub use error::*;

pub type Result<T> = core::result::Result<T, Error>;
pub type JsResult<T> = core::result::Result<T, wasm_bindgen::JsValue>;

#[cfg(feature = "panic-hook")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    #[cfg(feature = "tracing")]
    wasm_tracing::set_as_global_default();
}
