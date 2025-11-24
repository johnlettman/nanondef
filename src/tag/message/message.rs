use crate::tag::message::{record::Record, DecodeMessage, Iter, MessageRecords, RawMessage};

use crate::{validate::Validate, Error};
use alloc::vec::{IntoIter, Vec};

/// An owned, fully decoded NDEF message.
///
/// This type stores all [`Record`] values in a heap-allocated [`Vec`]. It is
/// the recommended representation for host environments where allocation is
/// available.
///
/// The best alternative for this in `no_std` environments is
/// [`HeaplessMessage`].
///
/// # Lifetimes
/// - `'m` -- lifetime of all record data.
///
/// # Notes
/// - All parsing happens up-front; iteration is zero-cost.
/// - The underlying buffer must outlive `'m`.
///
/// [`HeaplessMessage`]: crate::message::HeaplessMessage
#[derive(Debug, Clone, Default, derive_new::new)]
pub struct Message<'m>(pub Vec<Record<'m>>);

impl<'m> Message<'m> {
    /// Consumes `self` and returns the underlying [`Record`] vector.
    #[inline(always)]
    pub fn into_inner(self) -> Vec<Record<'m>> {
        self.0
    }
}

impl<'m> Validate for Message<'m> {
    type Error = Error;

    fn error(&self) -> Option<Self::Error> {
        self.0.iter().filter_map(|r| r.error()).next()
    }
}

impl<'m> MessageRecords<'m> for Message<'m> {
    type Error = Error;

    const DECODED: bool = true;

    #[inline(always)]
    fn records(&self) -> Result<&[Record<'m>], Self::Error> {
        Ok(self.0.as_slice())
    }

    #[inline(always)]
    fn iter_records<'i>(&'i self) -> Iter<'m, 'i> {
        self.0.as_slice().into()
    }

    #[inline(always)]
    fn push_record(&mut self, rec: Record<'m>) -> Result<(), Self::Error> {
        Ok(self.0.push(rec))
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

impl<'m> DecodeMessage<'m> for Message<'m> {
    #[inline(always)]
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        Self::try_from(bytes)
    }
}

impl<'m> AsRef<[Record<'m>]> for Message<'m> {
    /// Returns a slice of [`Record`] objects.
    #[inline(always)]
    fn as_ref(&self) -> &[Record<'m>] {
        self.0.as_slice()
    }
}

impl<'m> core::ops::Deref for Message<'m> {
    type Target = [Record<'m>];

    /// Deref an indexable view of [`Message`].
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m> From<Vec<Record<'m>>> for Message<'m> {
    #[inline(always)]
    fn from(v: Vec<Record<'m>>) -> Self {
        Self(v)
    }
}

impl<'m> TryFrom<&'m [u8]> for Message<'m> {
    type Error = Error;

    /// Attempts to decode a complete NDEF message from a byte slice.
    ///
    /// This wraps the bytes in a [`RawMessage`] and then delegates it to
    /// [`Self::try_from`].
    #[inline]
    fn try_from(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        let mut records = Vec::new();

        for record in RawMessage(bytes).iter_records() {
            records.push(record?);
        }

        Ok(Self(records))
    }
}

impl<'m> IntoIterator for Message<'m> {
    type Item = Record<'m>;
    type IntoIter = IntoIter<Self::Item>;

    /// Consumes the message and yields its records.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(feature = "serde")]
impl<'m> serde::Serialize for Message<'m>
where
    Record<'m>: serde::Serialize,
{
    /// Serializes the NDEF message as a sequence of records.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for record in self.0.iter() {
            seq.serialize_element(record)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de: 'm, 'm> serde::Deserialize<'de> for Message<'m>
where
    Record<'m>: serde::Deserialize<'de>,
{
    /// Deserializes the NDEF message from a sequence of records.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use core::marker::PhantomData;
        use serde::de::{SeqAccess, Visitor};

        struct MessageVisitor<'m>(PhantomData<&'m ()>);

        impl<'de: 'm, 'm> Visitor<'de> for MessageVisitor<'m>
        where
            Record<'m>: serde::Deserialize<'de>,
        {
            type Value = Message<'m>;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "a sequence of NDEF records")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut records = Vec::new();

                while let Some(record) = seq.next_element::<Record<'m>>()? {
                    records.push(record);
                }

                Ok(Message(records))
            }
        }

        deserializer.deserialize_seq(MessageVisitor(PhantomData))
    }
}
