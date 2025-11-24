mod heapless_message;
mod iter;
mod iter_raw;
mod iter_record;
mod raw_message;
pub mod record;

use crate::{
    error::ImplementationError,
    tag::message::record::{DecodeRecord, Record},
    trace::MaybeDebug,
};
use core::{
    any::type_name,
    fmt,
    fmt::{Debug, Display, Formatter},
};
pub use heapless_message::*;
pub use iter::*;
pub use iter_raw::*;
pub use iter_record::*;
pub use raw_message::*;

#[cfg(any(feature = "std", feature = "alloc"))]
mod message;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use message::*;

#[derive(Debug)]
pub struct NotDecodedError(pub &'static str);

impl Display for NotDecodedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported operation: {} does not contain decoded records", self.0)
    }
}

pub trait MessageRecords<'m>: Sized + MaybeDebug {
    type Error: ImplementationError;

    /// Is `true` if this message stores decoded [`Record`] objects internally.
    const DECODED: bool;

    /// Returns an iterator over decoded [`Record`] objects.
    fn iter_records<'i>(&'i self) -> Iter<'m, 'i>;

    /// If this message stores decoded [`Record`] objects, return them.
    fn records(&self) -> Result<&[Record<'m>], Self::Error> {
        let name: &'static str = type_name::<Self>();

        if Self::DECODED {
            unimplemented!("missing implementation of MessageRecords::records for {}", name)
        } else {
            Err(Self::Error::custom(NotDecodedError(name)))
        }
    }

    /// Attempt to append a record (only valid for [`MessageRecordContainer`]
    /// types).
    fn push_record(&mut self, _rec: Record<'m>) -> Result<(), Self::Error> {
        let name: &'static str = type_name::<Self>();

        if Self::DECODED {
            unimplemented!("missing implementation of MessageRecords::push_record for {}", name)
        } else {
            Err(Self::Error::custom(NotDecodedError(name)))
        }
    }

    /// Number of [`Record`] objects in this message.
    ///
    /// For [`MessageRecordContainer`] types, this is O(1).
    /// For [`MessageRecordSource`] types, this is O(n) and parses records on
    /// the fly.
    #[inline]
    fn len(&self) -> usize {
        if let Ok(slice) = self.records() {
            slice.len()
        } else {
            self.iter_records().count()
        }
    }

    /// Whether the message contains zero records.
    #[inline]
    fn is_empty(&self) -> bool {
        if let Ok(slice) = self.records() {
            slice.is_empty()
        } else {
            // stop early when iterating raw bytes
            self.iter_records().next().is_none()
        }
    }
}

pub trait DecodeMessage<'m>: Sized + MessageRecords<'m> {
    /// Decode this message from the TLV block bytes.
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error>;
}
