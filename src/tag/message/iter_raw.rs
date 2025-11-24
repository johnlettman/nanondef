use crate::tag::message::record::{DecodeRecord, Record};

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

/// An iterator over raw, undecoded [`Record`] values within a byte slice.
///
/// [`IterRaw`] reads records sequentially from the provided buffer,
/// validating headers and computing record boundaries on the fly.
///
/// This iterator yields:
/// - [`Ok`] with [`Record`] for each successfully parsed record
/// - [`Err`] with [`Error`] if a structural violation is found
///
/// Once an error occurs, the iterator marks itself as `done` and yields `None`
/// thereafter, matching the behavior of fallible iterators.
///
/// # Safety
/// This type does **not** assume the NDEF message is valid. Malformed messages
/// will produce a corresponding [`Error`] variant.
#[derive(Debug)]
pub struct IterRaw<'r> {
    bytes: &'r [u8],
    pos: usize,
    done: bool,
}

impl<'r> IterRaw<'r> {
    /// Creates a new iterator over a raw NDEF message.
    pub fn new(bytes: &'r [u8]) -> Self {
        Self { bytes, pos: 0, done: false }
    }
}

impl<'r> Default for IterRaw<'r> {
    /// Creates an empty iterator.
    fn default() -> Self {
        Self { bytes: &[], pos: 0, done: false }
    }
}

impl<'r> Iterator for IterRaw<'r> {
    type Item = crate::Result<Record<'r>>;

    #[cfg_attr(feature = "tracing",
        tracing::instrument(
            level = "trace",
            ret,
            skip(self),
            fields(
                pos = self.pos,
                done = self.done,
            )
        )
    )]
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.bytes.len();

        if self.done || self.pos >= len {
            return None;
        }

        let record_start = self.pos;
        match Record::decode_from_raw(&self.bytes[record_start..]) {
            Err(e) => {
                self.done = true;
                Some(Err(e))
            },

            Ok((pos, record)) => {
                self.pos = record_start + pos;
                Some(Ok(record))
            },
        }
    }
}

impl<'r> From<&'r [u8]> for IterRaw<'r> {
    #[inline(always)]
    fn from(bytes: &'r [u8]) -> Self {
        Self { bytes, pos: 0, done: false }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'r> From<&'r Vec<u8>> for IterRaw<'r> {
    #[inline(always)]
    fn from(bytes: &'r Vec<u8>) -> Self {
        Self { bytes, pos: 0, done: false }
    }
}

impl<'r, const L: usize> From<&'r heapless::Vec<u8, L>> for IterRaw<'r> {
    #[inline(always)]
    fn from(bytes: &'r heapless::Vec<u8, L>) -> Self {
        Self { bytes, pos: 0, done: false }
    }
}
