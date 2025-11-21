mod capability_container;
mod raw_tag;

use crate::message::DecodeMessage;
use crate::tlv::Block;
pub use capability_container::*;
pub use raw_tag::*;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
pub struct Tag<'t, M: DecodeMessage<'t>> {
    pub cc: CapabilityContainer,
    pub blocks: Vec<Block<'t, M>>,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'t, M: DecodeMessage<'t>> Tag<'t, M> {
    #[inline(always)]
    pub fn new(cc: CapabilityContainer, blocks: Vec<Block<'t, M>>) -> Self {
        Self { cc, blocks }
    }

    #[inline]
    pub fn decode(bytes: &'t [u8]) -> crate::Result<Self> {
        Self::try_from(RawTag::decode(bytes)?)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'t, M: DecodeMessage<'t>> TryFrom<&'t [u8]> for Tag<'t, M> {
    type Error = crate::Error;

    #[inline]
    fn try_from(bytes: &'t [u8]) -> Result<Self, Self::Error> {
        Self::try_from(RawTag::try_from(bytes)?)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'t, M: DecodeMessage<'t>> TryFrom<RawTag<'t>> for Tag<'t, M> {
    type Error = crate::Error;

    fn try_from(t: RawTag<'t>) -> Result<Self, Self::Error> {
        let mut blocks = Vec::new();

        for block in t.blocks::<M>() {
            blocks.push(block?);
        }

        Ok(Self { cc: t.cc, blocks })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'t, M: DecodeMessage<'t>> IntoIterator for Tag<'t, M> {
    type Item = Block<'t, M>;
    type IntoIter = alloc::vec::IntoIter<Block<'t, M>>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.blocks.into_iter()
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tag<'t, B: DecodeMessage<'t>, const L: usize> {
    pub cc: CapabilityContainer,
    pub blocks: heapless::Vec<B, L>,
    _marker: PhantomData<&'t ()>,
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'t, B: DecodeMessage<'t>, const L: usize> Tag<'t, B, L> {
    #[inline]
    pub fn new(cc: CapabilityContainer, blocks: heapless::Vec<B, L>) -> Self {
        Self { cc, blocks, _marker: PhantomData }
    }

    pub fn decode(bytes: &'t [u8]) -> crate::Result<Self> {
        Self::try_from(RawTag::decode(bytes)?)
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'t, B: DecodeMessage<'t>, const L: usize> TryFrom<RawTag<'t>> for Tag<'t, B, L> {
    type Error = crate::Error;

    fn try_from(t: RawTag<'t>) -> Result<Self, Self::Error> {
        let mut blocks = heapless::Vec::new();

        for block in t.blocks::<B>() {
            blocks.push(block?).map_err(|_| crate::Error::VecOutOfSpace(L))?;
        }

        Ok(Self { cc: t.cc, blocks, _marker: PhantomData })
    }
}
