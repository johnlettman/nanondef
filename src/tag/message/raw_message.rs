use crate::{
    tag::message::{DecodeMessage, Iter, IterRaw, MessageRecords},
    Error,
};

/// A borrowed NDEF message represented as a raw byte slice.
///
/// [`RawMessage`] performs no decoding itself; instead, it allows iteration
/// over raw NDEF records via [`RawIter`] and translation into fully decoded
/// structures like [`Message`] or [`HeaplessMessage`].
///
/// # Lifetimes
/// - `'m`: lifetime of all record data.
///
/// # Notes
/// - Zero-cost wrapper around a slice of [`u8`].
/// - Ideal for parsing messages incrementally.
/// - Produces fallible iterators for structural validation.
#[derive(Debug, Clone, derive_new::new)]
pub struct RawMessage<'m>(pub &'m [u8]);

impl<'m> RawMessage<'m> {
    /// Returns the number of bytes in the underlying message.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the message has length 0.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Consumes `self` and returns the underlying byte slice.
    #[inline(always)]
    pub fn into_inner(self) -> &'m [u8] {
        self.0
    }
}

impl<'m> MessageRecords<'m> for RawMessage<'m> {
    type Error = Error;

    const DECODED: bool = false;

    #[inline(always)]
    fn iter_records<'i>(&'i self) -> Iter<'m, 'i> {
        Iter::Raw(IterRaw::new(self.0))
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.iter_records().count()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'m> DecodeMessage<'m> for RawMessage<'m> {
    /// Wraps a byte slice as a [`RawMessage`] without validation.
    ///
    /// Structural integrity is checked lazily during iteration over records.
    #[inline(always)]
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        Ok(Self(bytes))
    }
}

impl<'m> Default for RawMessage<'m> {
    /// Returns an empty [`RawMessage`].
    #[inline(always)]
    fn default() -> Self {
        Self(&[])
    }
}

impl<'m> AsRef<[u8]> for RawMessage<'m> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.0
    }
}

impl<'m> core::ops::Deref for RawMessage<'m> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'m> From<&'m [u8]> for RawMessage<'m> {
    fn from(buf: &'m [u8]) -> Self {
        Self(buf)
    }
}
