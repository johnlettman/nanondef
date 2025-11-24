use crate::{error::Truncated, Encode, Error};
use crate::tag::{Features, Version};

use crate::{
    validate::Validate,
    Error::{BufferTooSmall, CapabilityContainerTruncated},
};


#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

/// Represents an NFC Capability Container (CC) structure.
///
/// The Capability Container is a 4-byte block that appears at the beginning
/// of many NFC Forum tag types (e.g., Type 2 / Type 4 Tags), defining the
/// memory layout and feature support of the tag.
///
/// The structure is parsed as:
///
/// | byte 0 | byte 1  | byte 2        | byte 3   |
/// |--------|---------|---------------|----------|
/// | Magic  | Version | Size (blocks) | Features |
///
/// This crate treats the CC as a simple 4-byte fixed-size descriptor and
/// performs no interpretation of “blocks” or feature bits beyond storing
/// their values.
///
///
/// ```rust
/// use nanondef::CapabilityContainer;
/// let cc = CapabilityContainer::new(0xE1, 0x10, 0x06, 0x00);
/// let (b1, b2, b3, b4) = cc.into();
///
/// assert_eq!(cc.b1, 0xE1);
/// assert_eq!(cc.b1, 0x10);
/// assert_eq!(cc.b1, 0x06);
/// assert_eq!(cc.b1, 0x00);
/// ```
///
/// # Decoding
/// The CC can be decoded from:
///
/// - `&[u8]`
/// - [`Vec`] of `u8` (with the `"std"` or ``alloc`` features)
/// - [`heapless::Vec`] of `u8` (`L` length)
///
/// All decoding paths require **at least 4 bytes**, otherwise a
/// [`CapabilityContainerTruncated`](crate::Error::CapabilityContainerTruncated)
/// error is returned.
///
/// ```rust
/// # use nanondef::{CapabilityContainer, Error};
/// let bytes = [0xE1, 0x10, 0x06, 0x00];
/// let (cc, used) = CapabilityContainer::decode(&bytes).unwrap();
///
/// assert_eq!(used, 4);
/// assert_eq!(cc.magic, 0xE1);
/// assert_eq!(cc.version, 0x10);
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, derive_new::new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct CapabilityContainer {
    /// Magic byte identifying the Capability Container.
    pub magic: u8,

    /// CC version field.
    pub version: Version,

    /// Size of the NDEF memory area in 8-byte blocks.
    pub size_blocks: u8,

    /// Feature bitfield.
    pub features: Features,
}

impl CapabilityContainer {
    /// Size of the NDEF memory area in bytes.
    #[inline(always)]
    pub const fn size_ndef(&self) -> usize {
        self.size_blocks as usize * 8
    }

    #[inline(always)]
    pub const fn with_size_ndef(self, size: usize) -> Self {
        Self {
            magic: self.magic,
            version: self.version,
            size_blocks: (size / 8) as u8,
            features: self.features,
        }
    }

    #[inline(always)]
    pub const fn with_size_blocks(self, size: u8) -> Self {
        Self {
            magic: self.magic,
            version: self.version,
            size_blocks: size,
            features: self.features,
        }
    }
}

impl Validate for CapabilityContainer {
    type Error = Error;
}

/// Implements [`Encode`] for a [`CapabilityContainer`], producing the
/// canonical 4-byte NFC Capability Container block.
///
/// The encoded layout is:
///
/// | byte 0 | byte 1  | byte 2        | byte 3   |
/// |--------|---------|---------------|----------|
/// | Magic  | Version | Size (blocks) | Features |
impl Encode for CapabilityContainer {
    /// Capability Containers always encode as 4 bytes.
    #[inline(always)]
    fn encoded_len(&self) -> usize {
        4
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = buf.len();
        if len < 4 {
            return Err(BufferTooSmall { got: len, want: 4 });
        }

        buf[0] = self.magic;
        buf[1] = self.version.into();
        buf[2] = self.size_blocks;
        buf[3] = self.features.into();

        Ok(4)
    }
}

impl From<[u8; 4]> for CapabilityContainer {
    #[inline(always)]
    fn from(bytes: [u8; 4]) -> Self {
        Self::new(bytes[0], bytes[1].into(), bytes[2], bytes[3].into())
    }
}

impl From<CapabilityContainer> for [u8; 4] {
    #[inline(always)]
    fn from(cc: CapabilityContainer) -> Self {
        [cc.magic, cc.version.into(), cc.size_blocks, cc.features.into()]
    }
}

impl TryFrom<&[u8]> for CapabilityContainer {
    type Error = Error;

    /// Attempts to decode a Capability Container from a byte slice.
    ///
    /// The slice must contain **at least 4 bytes**, corresponding to the
    /// `magic`, `version`, `size_blocks`, `features` fields. Extra bytes are
    /// ignored.
    ///
    /// # Errors
    /// - [`CapabilityContainerTruncated`][Error::CapabilityContainerTruncated]
    ///     if the slice is shorter than 4 bytes.
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let len = bytes.len();

        if len < 4 {
            return Err(CapabilityContainerTruncated(Truncated { got: len, want: 4 }));
        }

        Ok(Self::new(bytes[0], bytes[1].into(), bytes[2], bytes[3].into()))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl TryFrom<&Vec<u8>> for CapabilityContainer {
    type Error = Error;

    /// Decodes a Capability Container from an allocated [`Vec`] of `u8`.
    ///
    /// Delegates to the `&[u8]` implementation.
    #[inline(always)]
    fn try_from(bytes: &Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}

impl<const L: usize> TryFrom<&heapless::Vec<u8, L>> for CapabilityContainer {
    type Error = Error;

    /// Decodes a Capability Container from a fixed-capacity [`heapless::Vec`]
    /// of `u8`.
    ///
    /// Delegates to the `&[u8]` implementation.
    #[inline(always)]
    fn try_from(bytes: &heapless::Vec<u8, L>) -> Result<Self, Self::Error> {
        Self::try_from(bytes.as_slice())
    }
}
