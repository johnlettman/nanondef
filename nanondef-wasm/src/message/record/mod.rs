mod payload;
mod raw_record;
mod record_kind;
mod record_payload;
mod header;

use nanondef::message::record;
pub use raw_record::*;
pub use record_kind::*;
pub use record_payload::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Record {
    pub header: record::Header,

    #[wasm_bindgen(getter_with_clone)]
    pub ty: String,

    #[wasm_bindgen(getter_with_clone)]
    pub payload: RecordPayload,
}

impl From<&record::Record<'_>> for Record {
    fn from(r: &record::Record<'_>) -> Self {
        let (header, ty, payload) = match r {
            record::Record::Raw(raw) => (raw.header, raw.ty.into(), raw.into()),
            record::Record::Uri(uri) => (uri.header, uri.ty.into(), uri.into()),
        };

        Self { header, ty, payload }
    }
}
