pub mod record;

use crate::message::record::Record;
use nanondef::message;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message(Vec<Record>);

#[wasm_bindgen]
impl Message {
    #[wasm_bindgen]
    #[inline]
    pub fn new(records: Vec<Record>) -> Self {
        Self(records)
    }

    #[wasm_bindgen]
    #[inline]
    pub fn decode(bytes: &[u8]) -> crate::Result<Self> {
        Ok(Self::try_from(bytes)?)
    }

    #[wasm_bindgen(getter)]
    pub fn records(&self) -> Vec<Record> {
        self.0.clone()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let m = message::Message::try_from(bytes)?;
        Ok(Self::from(&m))
    }
}

impl From<&message::Message<'_>> for Message {
    #[inline]
    fn from(m: &message::Message) -> Self {
        Self(m.0.iter().map(|r| r.into()).collect())
    }
}

impl TryFrom<&message::RawMessage<'_>> for Message {
    type Error = crate::Error;

    #[inline]
    fn try_from(m: &message::RawMessage) -> Result<Self, Self::Error> {
        let m = message::Message::try_from(m)?;
        Ok(Self::from(&m))
    }
}
