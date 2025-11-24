use crate::{
    tag::message::{record::Record, IterRaw, IterRecord},
    Error,
};
use core::any::type_name;

/// A unified iterator over [`Record`] values.
///
/// This wrapper abstracts over two concrete iterator types:
/// - [`RawIter`] decodes records on demand from a byte slice
/// - [`DataIter`] yields already-decoded records from a slice or vec
///
/// # Lifetimes
/// - `'r`: lifetime of the underlying record data.
/// - `'i`: lifetime of the slice being iterated.
pub enum Iter<'r, 'i> {
    /// Decode records directly from a raw byte slice.
    Raw(IterRaw<'r>),

    /// Iterate over an in-memory list of decoded records.
    Decoded(IterRecord<'r, 'i>),
}

impl<'r, 'i> Iterator for Iter<'r, 'i> {
    type Item = crate::Result<Record<'r>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Decoded(iter) => iter.next(),
            Self::Raw(iter) => iter.next(),
        }
    }
}

impl<'r, 'i> From<&'r [u8]> for Iter<'r, 'i> {
    #[inline]
    fn from(bytes: &'r [u8]) -> Self {
        Self::Raw(bytes.into())
    }
}

impl<'r, 'i> From<&'i [Record<'r>]> for Iter<'r, 'i> {
    #[inline]
    fn from(s: &'i [Record<'r>]) -> Self {
        Self::Decoded(s.into())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'r, 'i> From<&'i Vec<Record<'r>>> for Iter<'r, 'i> {
    #[inline]
    fn from(vec: &'i Vec<Record<'r>>) -> Self {
        Self::Decoded(vec.into())
    }
}

impl<'r, 'i, const L: usize> From<&'i heapless::Vec<Record<'r>, L>> for Iter<'r, 'i> {
    #[inline]
    fn from(vec: &'i heapless::Vec<Record<'r>, L>) -> Self {
        Self::Decoded(vec.into())
    }
}

impl<'r, 'i> From<IterRaw<'r>> for Iter<'r, 'i> {
    #[inline(always)]
    fn from(i: IterRaw<'r>) -> Self {
        Self::Raw(i)
    }
}

impl<'r, 'i> TryFrom<Iter<'r, 'i>> for IterRaw<'r> {
    type Error = Error;

    #[inline]
    fn try_from(i: Iter<'r, 'i>) -> Result<Self, Self::Error> {
        match i {
            Iter::Raw(i) => Ok(i),
            Iter::Decoded(_) => Err(Error::WrongRecordIter {
                got: type_name::<IterRecord>(),
                want: type_name::<IterRaw>(),
            }),
        }
    }
}

impl<'r, 'i> From<IterRecord<'r, 'i>> for Iter<'r, 'i> {
    #[inline(always)]
    fn from(i: IterRecord<'r, 'i>) -> Self {
        Self::Decoded(i)
    }
}

impl<'r, 'i> TryFrom<Iter<'r, 'i>> for IterRecord<'r, 'i> {
    type Error = Error;

    #[inline]
    fn try_from(i: Iter<'r, 'i>) -> Result<Self, Self::Error> {
        match i {
            Iter::Raw(_) => Err(Error::WrongRecordIter {
                got: type_name::<IterRaw>(),
                want: type_name::<IterRecord>(),
            }),
            Iter::Decoded(i) => Ok(i),
        }
    }
}
