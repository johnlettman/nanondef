use crate::CapabilityContainer;
use crate::message::DecodeMessage;
use crate::tlv::{BlockIter};

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

#[derive(Debug, Clone, derive_new::new)]
pub struct RawTag<'t> {
    pub cc: CapabilityContainer,
    pub bytes: &'t [u8]
}

impl<'t> RawTag<'t> {
    #[inline(always)]
    pub fn decode(bytes: &'t [u8]) -> crate::Result<Self> {
        Self::try_from(bytes)
    }

    pub fn blocks<B: DecodeMessage<'t>>(&self) -> BlockIter<'t, B> {
        BlockIter::new(self.bytes)
    }
}

impl<'t> TryFrom<&'t [u8]> for RawTag<'t> {
    type Error = crate::Error;

    fn try_from(bytes: &'t [u8]) -> Result<Self, Self::Error> {
        let (cc, pos) = CapabilityContainer::decode(bytes)?;
        Ok(Self { cc, bytes: &bytes[pos..] })
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
