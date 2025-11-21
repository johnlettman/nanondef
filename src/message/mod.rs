mod raw_message;
pub mod record;

use crate::message::record::Record;
pub use raw_message::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;
use crate::Range;

#[cfg(not(feature = "tracing"))]
pub trait DecodeMessage<'b>: Sized {
    type Error: core::fmt::Display;

    /// Decode this message from the TLV value bytes (not including the tag or
    /// length).
    fn decode_block(bytes: &'b [u8]) -> Result<Self, Self::Error>;
}

#[cfg(feature = "tracing")]
pub trait DecodeMessage<'b>: Sized + core::fmt::Debug {
    type Error: core::fmt::Display + core::fmt::Debug;

    /// Decode this message from the TLV value bytes (not including the tag or
    /// length).
    fn decode_message(bytes: &'b [u8]) -> Result<Self, Self::Error>;
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Message<'m>(Vec<(Record<'m>, Range)>);


#[cfg(any(feature = "std", feature = "alloc"))]
impl<'m> Message<'m> {
    #[inline(always)]
    pub const fn new(records: Vec<Record<'m>>) -> Self {
        Self(records)
    }

    #[inline(always)]
    pub fn decode(bytes: &'m [u8]) -> crate::Result<Self> {
        Self::try_from(bytes)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'m> TryFrom<&RawMessage<'m>> for Message<'m> {
    type Error = crate::Error;

    fn try_from(m: &RawMessage<'m>) -> Result<Self, Self::Error> {
        let mut records = Vec::new();

        for record in m.records() {
            records.push(record?);
        }

        Ok(Self(records))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'m> TryFrom<&'m [u8]> for Message<'m> {
    type Error = crate::Error;

    #[inline]
    fn try_from(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        Self::try_from(&RawMessage(bytes))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'m> IntoIterator for Message<'m> {
    type Item = Record<'m>;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'m> DecodeMessage<'m> for Message<'m> {
    type Error = crate::Error;

    const BLOCK_TAG: BlockTag = BlockTag::Message;

    #[inline(always)]
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        Self::try_from(bytes)
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
pub struct Message<'m, const L: usize>(pub heapless::Vec<Record<'m>, L>);

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'m, const L: usize> Message<'m, L> {
    #[inline(always)]
    pub const fn new(records: heapless::Vec<Record<'m>, L>) -> Self {
        Self(records)
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'m, const L: usize> TryFrom<RawMessage<'m>> for Message<'m, L> {
    type Error = crate::Error;

    fn try_from(m: RawMessage<'m>) -> Result<Self, Self::Error> {
        let mut records = heapless::Vec::new();

        for record in m.records() {
            records.push(record?).map_err(|_| crate::Error::VecOutOfSpace(L))?;
        }

        Ok(Self(records))
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'m, const L: usize> IntoIterator for Message<'m, L> {
    type Item = Record<'m>;
    type IntoIter = heapless::vec::IntoIter<Self::Item, L, usize>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
