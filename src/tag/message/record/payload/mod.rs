mod uri;
mod uri_prefix_map;

use crate::{
    error::ImplementationError,
    tag::message::record::{Flags, Header},
    trace::MaybeDebug,
    Error,
};
pub use uri::*;

/// A trait for decoding typed payloads from a raw NDEF record [`RawRecord`].
///
/// This trait defines the interface for converting the raw payload bytes
/// contained in a [`RawRecord`] into a strongly-typed Rust value.
/// Implementors are responsible for validating record metadata (TNF, type,
/// flags, expected payload format) in the [`decodes`] function and for
/// performing any necessary parsing.
///
/// The lifetime `'p` ties the decoded payload to the lifetime of the underlying
/// [`RawRecord`] bytes. Implementors may choose to decode in a *zero-copy*
/// fashion (borrowing substrings/slices directly from the record), or they may
/// allocate new owned values if desired.
///
/// [`decodes`]: DecodePayload::decodes
pub trait DecodePayload<'p>: Sized + MaybeDebug {
    /// The error type returned when decoding fails.
    ///
    /// Must implement [`Display`] and optionally [`Debug`] depending on the
    /// active `"tracing"` feature set via [`MaybeDebug`].
    type Error: ImplementationError;

    /// Quickly determines whether this payload type can decode the provided
    /// record.
    ///
    /// The default implementation always returns `true`, meaning implementors
    /// should override this when their format requires particular TNF/type
    /// field matching.
    fn decodes_record(flags: &Flags, header: &Header) -> bool;

    /// Fully decodes the payload contained in the given [`RawRecord`].
    fn decode_payload(bytes: &'p [u8], flags: &Flags, header: &Header)
        -> Result<Self, Self::Error>;
}

impl<'p> DecodePayload<'p> for &'p [u8] {
    type Error = Error;

    #[inline(always)]
    fn decodes_record(_: &Flags, _: &Header) -> bool {
        true
    }

    #[inline(always)]
    fn decode_payload(bytes: &'p [u8], _: &Flags, _: &Header) -> Result<Self, Self::Error> {
        Ok(bytes)
    }
}
