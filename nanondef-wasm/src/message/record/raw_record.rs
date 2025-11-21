use js_sys::Uint8Array;
use nanondef::message::record;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawRecord(#[serde(with = "serde_bytes")] Vec<u8>);

#[wasm_bindgen]
impl RawRecord {
    #[wasm_bindgen(constructor, js_name = new)]
    pub fn new(data: Vec<u8>) -> RawRecord {
        RawRecord(data)
    }

    #[wasm_bindgen(getter, js_name = bytes)]
    pub fn bytes(&self) -> Uint8Array {
        let array = unsafe { Uint8Array::view(&self.0) };

        array.slice(0, self.0.len() as u32)
    }
}

impl From<&record::RawRecord<'_>> for RawRecord {
    fn from(r: &record::RawRecord) -> Self {
        Self(r.bytes.into())
    }
}
