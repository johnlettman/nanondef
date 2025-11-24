use crate::{
    tag::message::{record::Record, DecodeMessage, Iter, MessageRecords, RawMessage},
    Error,
    Error::VecOutOfSpace,
};

/// A fixed-capacity, heapless, fully decoded NDEF message.
///
/// This type stores a bounded number of decoded [`Record`] values using a
/// [`heapless::Vec`]. It is ideal for embedded `no_std` targets where dynamic
/// allocation is unavailable.
///
/// <div class="warning">
///
/// If the message contains more than `L` records, decoding fails with
/// [`VecOutOfSpace`].
///
/// </div>
///
/// # Lifetimes
/// - `'m`: lifetime of all record data.
///
/// # Type Parameters
/// - `L`: maximum number of records in the message.
///
/// # Notes
/// - All parsing happens up-front; iteration is zero-cost.
/// - The underlying buffer must outlive `'m`.
#[derive(Debug, Default, Clone)]
pub struct HeaplessMessage<'m, const L: usize>(pub heapless::Vec<Record<'m>, L>);

impl<'m, const L: usize> HeaplessMessage<'m, L> {
    /// Creates a new [`HeaplessMessage`] from a pre-existing [`heapless::Vec`].
    #[inline(always)]
    pub const fn new(records: heapless::Vec<Record<'m>, L>) -> Self {
        Self(records)
    }

    /// Consumes `self` and returns the underlying [`Record`] vector.
    #[inline(always)]
    pub fn into_inner(self) -> heapless::Vec<Record<'m>, L> {
        self.0
    }
}

impl<'m, const L: usize> MessageRecords<'m> for HeaplessMessage<'m, L> {
    type Error = Error;

    const DECODED: bool = true;

    #[inline(always)]
    fn iter_records<'i>(&'i self) -> Iter<'m, 'i> {
        (&self.0).into()
    }

    #[inline(always)]
    fn records(&self) -> Result<&[Record<'m>], Self::Error> {
        Ok(self.0.as_slice())
    }

    fn push_record(&mut self, rec: Record<'m>) -> Result<(), Self::Error> {
        self.0.push(rec).map_err(|_| VecOutOfSpace(L))
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'m, const L: usize> DecodeMessage<'m> for HeaplessMessage<'m, L> {
    /// Decodes a heapless NDEF message from a byte slice.
    ///
    /// This wraps the bytes in a [`RawMessage`] and uses the `TryFrom` impl.
    #[inline(always)]
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        RawMessage(bytes).try_into()
    }
}

impl<'m, const L: usize> AsRef<[Record<'m>]> for HeaplessMessage<'m, L> {
    /// Returns a slice of [`Record`] objects.
    #[inline(always)]
    fn as_ref(&self) -> &[Record<'m>] {
        self.0.as_slice()
    }
}

impl<'m, const L: usize> core::ops::Deref for HeaplessMessage<'m, L> {
    type Target = [Record<'m>];

    /// Deref an indexable view of [`HeaplessMessage`].
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'m, const L: usize> TryFrom<RawMessage<'m>> for HeaplessMessage<'m, L> {
    type Error = Error;

    fn try_from(m: RawMessage<'m>) -> Result<Self, Self::Error> {
        let mut records = heapless::Vec::new();

        for record in m.iter_records() {
            records.push(record?).map_err(|_| VecOutOfSpace(L))?;
        }

        Ok(Self(records))
    }
}

impl<'m, const L: usize> IntoIterator for HeaplessMessage<'m, L> {
    type Item = Record<'m>;
    type IntoIter = heapless::vec::IntoIter<Self::Item, L, usize>;

    /// Consumes the message and yields its records.
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(feature = "serde")]
impl<'m, const L: usize> serde::Serialize for HeaplessMessage<'m, L>
where
    Record<'m>: serde::Serialize,
{
    /// Serializes the NDEF message as a sequence of records.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for record in self.0.iter() {
            seq.serialize_element(record)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de: 'm, 'm, const L: usize> serde::Deserialize<'de> for HeaplessMessage<'m, L>
where
    Record<'m>: serde::Deserialize<'de>,
{
    /// Deserializes the NDEF message from a sequence of records.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::marker::PhantomData;
        use serde::de::{Error, SeqAccess, Visitor};

        struct HeaplessMessageVisitor<'m, const L: usize>(PhantomData<&'m ()>);

        impl<'de: 'm, 'm, const L: usize> Visitor<'de> for HeaplessMessageVisitor<'m, L>
        where
            Record<'m>: serde::Deserialize<'de>,
        {
            type Value = HeaplessMessage<'m, L>;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "a sequence of NDEF records (< {} items)", L)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut vec = heapless::Vec::new();

                while let Some(record) = seq.next_element::<Record<'m>>()? {
                    vec.push(record).map_err(|_| Error::custom(VecOutOfSpace(L)))?;
                }

                Ok(HeaplessMessage(vec))
            }
        }

        deserializer.deserialize_seq(HeaplessMessageVisitor::<L>(PhantomData))
    }
}
