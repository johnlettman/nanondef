//! Error types and error-construction helpers for NDEF encoding and decoding.

use core::{
    fmt,
    fmt::{Debug, Display, Formatter},
};
use pastey::paste;

#[cfg(feature = "alloc")]
use alloc::{string::String, string::ToString};

/// The top-level error type for all NDEF operations.
///
/// This enum aggregates all errors that may occur during:
/// - message-level decoding ([`DecodeMessage`])
/// - record-level decoding ([`DecodeRecord`])
/// - payload-level decoding ([`DecodePayload`])
/// - payload-level encoding ([`Encode`])
///
/// It also includes utility errors:
/// - UTF-8 conversion errors
/// - buffer size mismatches
/// - truncated buffers
/// - invalid flags, prefix codes, or record structure
///
/// # Alloc vs. no-alloc behavior
/// - With the `"std"` or `"alloc"` features: section errors carry an owned
///   [`String`].
/// - Without allocation: section errors use static `str` messages.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // General Errors
    // ==============
    /// The error type for errors that can never happen.
    #[error("infallible error triggered!")]
    Infallible(#[from] core::convert::Infallible),

    /// A static vector (e.g., [`heapless::Vec`]) ran out of capacity during
    /// encoding.
    ///
    /// Common in `no_std` or embedded environments where capacity is fixed.
    ///
    /// The parameter indicates the number of elements currently stored.
    #[error("vec is out of space at {0} entries")]
    VecOutOfSpace(usize),

    /// The destination buffer passed to an encoder is too small.
    #[error("buffer too small at {got} bytes, need {want} bytes")]
    BufferTooSmall {
        /// The size of the provided buffer.
        got: usize,

        /// The required size.
        want: usize,
    },

    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("bad field: {0}")]
    BadField(&'static str),

    #[error("unsupported operation: {0:?}")]
    Unsupported(Option<&'static str>),

    #[error("implementation error: {0}")]
    ImplementationError(ErrorMessage),

    // Tag Errors
    // ==========
    /// The NDEF Capability Container (CC) is shorter than required.
    #[error("CC is truncated: {0}")]
    CapabilityContainerTruncated(Truncated),

    /// A raw block of TLV data was shorter than required for decoding.
    #[error("TLV block is truncated: {0}")]
    BlockTruncated(Truncated),

    /// The version is too large.
    #[error("CC version {field} field too large, got {got}")]
    VersionTooLarge {
        field: &'static str,
        got: u8,
    },

    // Message Errors
    // ==============

    // Record Errors
    // =============
    /// A trait-level error produced by [`DecodeRecord`].
    #[error("NDEF record error: {0}")]
    RecordError(ErrorMessage),

    /// An NDEF record was expected, but the input slice is empty.
    #[error("empty NDEF record")]
    EmptyRecord,

    /// The NDEF record is shorter than required.
    #[error("NDEF record is truncated: {0}")]
    RecordTruncated(Truncated),

    /// The ID string of an NDEF record exceeds the maximum allowed by the NDEF
    /// specification.
    #[error("NDEF record ID length of {0} bytes is too long")]
    RecordIdTooLong(usize),

    /// The type string of an NDEF record exceeds the maximum allowed by the
    /// NDEF specification.
    #[error("NDEF record type length of {0} bytes is too long")]
    RecordTyTooLong(usize),

    /// A UTF-8 decoding error occurred.
    #[error("UTF-8 error in NDEF type field: {0}")]
    RecordTyUtf8Error(#[source] core::str::Utf8Error),

    /// The incorrect NDEF record iterator was used.
    #[error("wrong NDEF record iterator called: called {got}, want {want}")]
    WrongRecordIter {
        /// The called NDEF record iterator.
        got: &'static str,

        /// The required NDEF record iterator.
        want: &'static str,
    },

    // Payload Errors
    // ==============
    /// A trait-level error produced by [`DecodePayload`].
    #[error("NDEF payload error: {0}")]
    PayloadError(ErrorMessage),

    /// The payload portion of an NDEF record is shorter than required.
    #[error("NDEF payload is truncated: {0}")]
    PayloadTruncated(Truncated),

    /// The payload format does not match what the decoder expects.
    #[error("invalid payload for record: {0}")]
    PayloadInvalid(&'static str),

    /// The TNF (Type Name Format) of a record does not match what a payload
    /// decoder expects.
    #[error("invalid NDEF record TNF for payload: have {got}, want {want}")]
    RecordTnfInvalid {
        /// The actual TNF value encountered.
        got: crate::tag::message::record::Tnf,

        /// The required TNF value for this record.
        want: crate::tag::message::record::Tnf,
    },

    /// The NDEF type field does not match the expected type for the decoder.
    #[error("invalid NDEF type for {0}")]
    RecordTyInvalidForPayload(&'static str),

    // Payload Errors: URI Payload
    // ===========================
    /// A UTF-8 decoding error occurred when interpreting the textual part of a
    /// URI payload.
    #[error("UTF-8 error in URI: {0}")]
    UriPayloadUtf8Error(#[source] core::str::Utf8Error),

    /// A prefix code in an NDEF URI record refers to a prefix outside the
    /// allowed NFC RTD-URI prefix map.
    ///
    /// See: [`UriPayload::PREFIX_MAP`][crate::message::record::payload::UriPayload::PREFIX_MAP]
    #[error("invalid URI prefix code: {0}")]
    UriPayloadPrefixInvalid(usize),

    /// The prefix in an NDEF URI record is not matched to a code in the allowed
    /// NFC RTD-URI prefix map.
    ///
    /// See: [`UriPayload::PREFIX_MAP`][crate::message::record::payload::UriPayload::PREFIX_MAP]
    #[error("the URI prefix was not found in the prefix map: {0}")]
    UriPayloadPrefixNotMatched(&'static str),

    /// A TLV termination marker (`0xFE`) was encountered before any TLV block.
    #[error("unexpected TLV terminator before TLV block")]
    TerminatorBeforeBlock,
}

/// Indicates that a buffer or field was too short to contain the expected data.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Truncated {
    /// The actual buffer or field length.
    pub got: usize,

    /// The required buffer or field length.
    pub want: usize,
}

impl Display for Truncated {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "have {} bytes, want {} bytes", self.got, self.want)
    }
}

/// Generates an error constructor for truncated buffers.
macro_rules! truncated_error {
    ($error:ident, $function:ident) => {
        impl Error {
            #[inline]
            pub fn $function(len: usize, want: usize) -> Self {
                Self::$error(Truncated { got: len, want })
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorMessage {
    Static(&'static str, Option<usize>),

    #[cfg(any(feature = "std", feature = "alloc"))]
    Owned(String, Option<usize>),
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static(s, pos) => match pos {
                Some(pos) => write!(f, "{} at {} bytes", s, pos),
                None => write!(f, "{}", s),
            },

            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Owned(s, pos) => match pos {
                Some(pos) => write!(f, "{} at {} bytes", s, pos),
                None => write!(f, "{}", s),
            },
        }
    }
}

macro_rules! declare_error_trait {
    ($error:ident: $( $super:tt )+ ) => {
        pub trait $error: $( $super )+ {
            #[inline(always)]
            fn custom<E: core::fmt::Display>(e: E) -> Self {
                Self::custom_at(e, None)
            }

            #[inline(always)]
            fn custom_at<E: core::fmt::Display>(_e: E, _pos: Option<usize>) -> Self {
                unimplemented!()
            }
        }

        impl $error for crate::Error {
            fn custom_at<E: core::fmt::Display>(e: E, pos: Option<usize>) -> Self {
                #[cfg(not(any(feature = "std", feature = "alloc")))]
                return Self::ImplementationError(ErrorMessage::Static(core::any::type_name::<E>(), pos));

                #[cfg(any(feature = "std", feature = "alloc"))]
                return Self::ImplementationError(ErrorMessage::Owned(e.to_string(), pos));
            }
        }

        paste! {
            impl crate::Error {
                #[inline]
                pub fn [<$error:snake>]<E>(e: E) -> Self
                where
                    E: $error + core::fmt::Display,
                {
                    #[cfg(not(any(feature = "std", feature = "alloc")))]
                    return Self::$error(ErrorMessage::Static(core::any::type_name::<E>(), None));

                    #[cfg(any(feature = "std", feature = "alloc"))]
                    return Self::$error(ErrorMessage::Owned(e.to_string(), None));
                }

                #[inline]
                pub fn [<$error:snake _at>]<E>(e: E, pos: usize) -> Self
                where
                    E: $error + core::fmt::Display,
                {
                    #[cfg(not(any(feature = "std", feature = "alloc")))]
                    return Self::$error(ErrorMessage::Static(core::any::type_name::<E>(), Some(pos)));

                    #[cfg(any(feature = "std", feature = "alloc"))]
                    return Self::$error(ErrorMessage::Owned(e.to_string(), Some(pos)));
                }
            }
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
declare_error_trait!(ImplementationError: core::error::Error + Sized);

#[cfg(not(any(feature = "std", feature = "alloc")))]
declare_error_trait!(ImplementationError: Debug + Display + Sized);

truncated_error!(BlockTruncated, block_truncated);
truncated_error!(RecordTruncated, record_truncated);
truncated_error!(PayloadTruncated, payload_truncated);
