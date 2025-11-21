use nanondef::message::record::payload;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UriPayload {
    pub uri: String,
}

impl<'r> From<&payload::UriPayload<'r>> for UriPayload {
    fn from(p: &payload::UriPayload<'r>) -> Self {
        Self { uri: p.to_string() }
    }
}
