use crate::tag::message::record::Record;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

/// An iterator over already-decoded [`Record`] values stored in a slice.
///
/// [`IterRecord`] is used when iterating over a collection of decoded records.
/// It simply wraps a slice iterator ([`core::slice::Iter`]) and yields [`Ok`]
/// values, allowing it to be used interchangeably with [`IterRaw`] via
/// [`Iter`].
///
/// This iterator performs **no decoding**.
/// All records are already parsed.
///
/// # Lifetimes
/// - `'r` -- lifetime of the underlying record data.
/// - `'i` -- lifetime of the slice being iterated.
///
/// These can differ when a message borrows a buffer, but the record list itself
/// is owned (e.g., in a [`Vec`] or [`heapless::Vec`]).
///
/// [`Iter`]: crate::message::record::Iter
/// [`IterRaw`]: crate::message::record::IterRaw
#[derive(Debug, Clone)]
pub struct IterRecord<'r, 'i>(core::slice::Iter<'i, Record<'r>>);

impl<'r, 'i> Iterator for IterRecord<'r, 'i> {
    type Item = crate::Result<Record<'r>>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().cloned().map(Ok)
    }
}

impl<'r, 'i> From<&'i [Record<'r>]> for IterRecord<'r, 'i> {
    #[inline]
    fn from(s: &'i [Record<'r>]) -> Self {
        Self(s.iter())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'r, 'i> From<&'i Vec<Record<'r>>> for IterRecord<'r, 'i> {
    #[inline]
    fn from(vec: &'i Vec<Record<'r>>) -> Self {
        Self(vec.iter())
    }
}

impl<'r, 'i, const L: usize> From<&'i heapless::Vec<Record<'r>, L>> for IterRecord<'r, 'i> {
    #[inline]
    fn from(vec: &'i heapless::Vec<Record<'r>, L>) -> Self {
        Self(vec.iter())
    }
}
