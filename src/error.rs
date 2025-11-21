use crate::{message::record::{payload::DecodePayload, DecodeRecord}, Encode};
use crate::message::DecodeMessage;

#[cfg(feature = "alloc")]
use alloc::{string::String, string::ToString};
use paste::paste;
use crate::message::record::EncodeRecord;

macro_rules! section_error_fn {
    ($name:ident, $suffix:ident, $bound:tt) => {
        paste! {
            #[inline]
            pub fn [<$name:lower _ $suffix:lower>]<'_, T: $bound>(e: T::Error) -> Self {
                #[cfg(any(feature = "std", feature = "alloc"))]
                return Self::[<$name:camel Error>]([<$name:camel Error>]::Owned(e.to_string()));

                #[cfg(not(any(feature = "std", feature = "alloc")))]
                return Self::[<$name:camel Error>]([<$name:camel Error>]::Static("user error"));
            }
        }
    };
}


macro_rules! section_error {
    ($name:ident, $encode_bound:tt, $decode_bound:tt) => {
        impl Error {
            section_error_fn!($name, _decode_error, $encode_bound);
            section_error_fn!($name, _encode_error, $decode_bound<'_>);
        }

        paste!{
            #[derive(Debug)]
            pub enum [<$name:camel Error>] {
                #[cfg(any(feature = "std", feature = "alloc"))]
                Owned(String),
                Static(&'static str),
            }

            impl core::fmt::Display for [<$name:camel Error>] {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    match self {
                        #[cfg(any(feature = "std", feature = "alloc"))]
                        Self::Owned(s) => s.fmt(f),
                        Self::Static(s) => s.fmt(f),
                    }
                }
            }
        }
    };
}

macro_rules! truncated_error {
    ($error:ident, $function:ident) => {
        impl Error {
            #[inline]
            pub fn $function(len: usize, want: usize) -> Self {
                Self::$error(Truncated { len, want })
            }
        }
    };
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("vec is out of space at {0} entries")]
    VecOutOfSpace(usize),

    #[error("buffer too small at {len} bytes, need {want} bytes")]
    BufferTooSmall {
        len: usize,
        want: usize
    },

    #[error("tag size of {0} is too small")]
    TagTooSmall(usize),

    #[error("record ID length of {0} bytes is too long")]
    RecordIdTooLong(usize),

    #[error("record type length of {0} bytes is too long")]
    RecordTypeTooLong(usize),

    #[error("the capability container size of {0} is too small")]
    CapabilityContainerTooSmall(usize),

    #[error("unexpected TLV terminator before TLV block")]
    TerminatorBeforeBlock,

    #[error("empty NDEF record")]
    EmptyRecord,

    #[error("invalid NDEF record TNF for payload: have {tnf}, want {want}")]
    InvalidTnf {
        tnf: crate::message::record::Tnf,
        want: crate::message::record::Tnf,
    },

    #[error("invalid NDEF type for {0}")]
    InvalidType(&'static str),

    #[error("invalid payload for record: {0}")]
    InvalidPayload(&'static str),

    #[error("UTF-8 error in NDEF type field: {0}")]
    RecordTypeUtf8Error(#[source] core::str::Utf8Error),

    #[error("UTF-8 error in URI: {0}")]
    UriUtf8Error(#[source] core::str::Utf8Error),


    #[error("TLV message error: {0}")]
    MessageError(MessageError),

    #[error("NDEF record error: {0}")]
    RecordError(RecordError),

    #[error("NDEF payload error: {0}")]
    PayloadError(PayloadError),

    #[error("Block is truncated: {0}")]
    BlockTruncated(Truncated),

    #[error("Capability Container is truncated: {0}")]
    CapabilityContainerTruncated(Truncated),

    #[error("NDEF record is truncated: {0}")]
    RecordTruncated(Truncated),

    #[error("NDEF payload is truncated: {0}")]
    PayloadTruncated(Truncated),


    #[error("Invalid URI prefix code: {0}")]
    InvalidUriPrefix(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Truncated {
    pub len: usize,
    pub want: usize,
}

impl core::fmt::Display for Truncated {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "have {} bytes, want {} bytes", self.len, self.want)
    }
}

section_error!(message, Encode, DecodeMessage);
section_error!(record, EncodeRecord, DecodeRecord);
section_error!(payload, Encode, DecodePayload);

truncated_error!(BlockTruncated, block_truncated);
truncated_error!(RecordTruncated, record_truncated);
truncated_error!(PayloadTruncated, payload_truncated);
