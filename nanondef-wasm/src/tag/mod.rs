use crate::message::Message;
use nanondef::{self, message, CapabilityContainer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub cc: CapabilityContainer,

    #[wasm_bindgen(getter_with_clone)]
    pub blocks: Vec<Message>,
}

#[wasm_bindgen]
impl Tag {
    #[wasm_bindgen]
    pub fn new(cc: CapabilityContainer, blocks: Vec<Message>) -> Self {
        Self { cc, blocks }
    }

    #[wasm_bindgen]
    pub fn decode(bytes: &[u8]) -> crate::JsResult<Self> {
        Self::try_from(bytes).map_err(crate::Error::into)
    }
}

impl<'t> TryFrom<&'t [u8]> for Tag {
    type Error = crate::Error;

    fn try_from(bytes: &'t [u8]) -> Result<Self, Self::Error> {
        Ok(Tag::from(nanondef::Tag::<'t, message::Message<'t>>::try_from(bytes)?))
    }
}

impl<'t> TryFrom<nanondef::RawTag<'t>> for Tag {
    type Error = crate::Error;

    #[inline]
    fn try_from(t: nanondef::RawTag<'t>) -> Result<Self, Self::Error> {
        Ok(Tag::from(nanondef::Tag::<'t, message::Message<'t>>::try_from(t)?))
    }
}

impl<'t> From<nanondef::Tag<'t, message::Message<'t>>> for Tag {
    #[inline]
    fn from(t: nanondef::Tag<'t, message::Message<'t>>) -> Self {
        Self { cc: t.cc.into(), blocks: t.blocks.iter().map(|b| b.into()).collect() }
    }
}
