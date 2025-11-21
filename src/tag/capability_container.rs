use crate::Truncated;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, derive_new::new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct CapabilityContainer {
    pub magic: u8,
    pub version: u8,
    pub size_blocks: u8,
    pub features: u8,
}

impl CapabilityContainer {
    #[inline(always)]
    pub fn decode(bytes: &[u8]) -> crate::Result<(Self, usize)> {
        Ok((Self::try_from(bytes)?, 4))
    }
}

impl From<(u8, u8, u8, u8)> for CapabilityContainer {
    #[inline(always)]
    fn from((b1, b2, b3, b4): (u8, u8, u8, u8)) -> Self {
        Self::new(b1, b2, b3, b4)
    }
}

impl TryFrom<&[u8]> for CapabilityContainer
{
    type Error = crate::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let len = bytes.len();

        if len < 4 {
            return Err(crate::Error::CapabilityContainerTruncated (Truncated{ len, want: 4 }))
        }

        Ok(Self::new(bytes[0], bytes[1], bytes[2], bytes[3]))
    }
}

#[cfg(feature = "alloc")]
impl TryFrom<&Vec<u8>> for CapabilityContainer {
    type Error = crate::Error;

    #[inline(always)]
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

#[cfg(not(any(feature = "std", feature = "alloc")))]
impl<const L: usize> TryFrom<&heapless::Vec<u8, L>> for CapabilityContainer {
    type Error = crate::Error;

    #[inline(always)]
    fn try_from(bytes: &heapless::Vec<u8, L>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}
