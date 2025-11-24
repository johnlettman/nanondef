use crate::message::record::{payload::UriPayload, RawRecord, RecordKind};
use nanondef::message::{record, record::payload};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPayload {
    kind: RecordKind,

    raw: Option<RawRecord>,
    uri: Option<UriPayload>,
}

impl From<&record::Record<'_>> for RecordPayload {
    fn from(r: &record::Record<'_>) -> Self {
        match r {
            record::Record::Raw(raw) => raw.into(),
            record::Record::Uri(uri) => uri.into(),
        }
    }
}

impl From<&record::RawRecord<'_>> for RecordPayload {
    fn from(r: &record::RawRecord<'_>) -> Self {
        Self { kind: RecordKind::Raw, raw: Some(r.into()), uri: None }
    }
}

impl<'r> From<&record::Data<'r, payload::UriPayload<'r>>> for RecordPayload {
    fn from(r: &record::Data<'r, payload::UriPayload<'r>>) -> Self {
        Self { kind: RecordKind::Uri, raw: None, uri: Some((&r.payload).into()) }
    }
}
