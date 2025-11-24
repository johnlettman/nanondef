use crate::tag::{message::DecodeMessage, CapabilityContainer, IterBlock};
use crate::Encode;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

/// A raw TLV tag consisting of a Capability Container and a borrowed byte
/// slice.
///
/// Parsing into [`HeaplessTag`] or the allocated [`Tag`] type is performed
/// via [`blocks`], which produces a [`IterBlock`].
///
/// This type does **not** validate the TLV payload by itself.
///
/// [`blocks`]: RawTag::blocks
/// [`Tag`]: crate::Tag
/// [`HeaplessTag`]: crate::HeaplessTag
#[derive(Debug, Clone, derive_new::new)]
pub struct RawTag<'t> {
    pub cc: CapabilityContainer,
    pub bytes: &'t [u8],
}

impl<'t> RawTag<'t> {
    #[inline(always)]
    pub fn decode(bytes: &'t [u8]) -> crate::Result<Self> {
        Self::try_from(bytes)
    }

    pub fn blocks<B: DecodeMessage<'t>>(&self) -> IterBlock<'t, B> {
        IterBlock::new(self.bytes)
    }
}

impl<'t> TryFrom<&'t [u8]> for RawTag<'t> {
    type Error = crate::Error;

    fn try_from(bytes: &'t [u8]) -> Result<Self, Self::Error> {
        let cc = CapabilityContainer::try_from(bytes)?;
        Ok(Self { cc, bytes: &bytes[cc.encoded_len()..] })
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'t> TryFrom<&'t Vec<u8>> for RawTag<'t> {
    type Error = crate::Error;

    fn try_from(bytes: &'t Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<'t, const L: usize> TryFrom<&'t heapless::Vec<u8, L>> for RawTag<'t> {
    type Error = crate::Error;

    fn try_from(bytes: &'t heapless::Vec<u8, L>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}
