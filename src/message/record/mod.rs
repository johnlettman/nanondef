mod header;
pub mod payload;
mod payload_record;
mod raw_record;
mod record_iter;
mod tnf;

use crate::message::record::payload::DecodePayload;
use crate::trace::MaybeDebug;
use crate::Encode;
pub use header::*;
pub use payload_record::*;
pub use raw_record::*;
pub use record_iter::*;
pub use tnf::*;

pub trait EncodeRecord: Sized + Encode {
    fn header(&self, mb: bool, me: bool) -> Header;

    /// Encode the record into the given buffer.
    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool) -> Result<usize, Self::Error>;

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode_record(&self, mb: bool, me: bool) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode_record_into(&mut buf, mb, me)?;
        Ok(buf)
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode_record<const L: usize>(
        &self,
        mb: bool,
        me: bool,
    ) -> Result<heapless::Vec<u8, L>, Self::Error> {
        if L < self.encoded_len() {
            panic!("undersized encode Vec buffer");
        }

        let mut buf = heapless::Vec::new();
        self.encode_record_into(&mut buf, mb, me)?;
        Ok(buf)
    }
}

pub trait DecodeRecord<'r>: Sized + MaybeDebug {
    type Error: core::fmt::Display + MaybeDebug;

    fn decode_record_from(bytes: &'r [u8]) -> Result<(Self, usize), Self::Error>;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Record<'r> {
    Raw(RawRecord<'r>),
    Uri(PayloadRecord<'r, payload::UriPayload<'r>>),
}

impl<'r> Encode for Record<'r> {
    type Error = crate::Error;

    #[inline]
    fn encoded_len(&self) -> usize {
        match self {
            Self::Raw(raw) => raw.encoded_len(),
            Self::Uri(uri) => uri.encoded_len(),
        }
    }

    #[inline(always)]
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.encode_record_into(buf, false, false)
    }
}

impl<'r> EncodeRecord for Record<'r> {
    fn header(&self, mb: bool, me: bool) -> Header {
        match self {
            Self::Raw(raw) => raw.header(mb, me),
            Self::Uri(uri) => uri.header(mb, me),
        }
    }

    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool) -> Result<usize, Self::Error> {
        match self {
            Self::Raw(raw) => raw.encode_record_into(buf, mb, me),
            Self::Uri(uri) => uri.encode_record_into(buf, mb, me),
        }
    }
}

impl<'r> DecodeRecord<'r> for Record<'r> {
    type Error = crate::Error;

    fn decode_record_from(bytes: &'r [u8]) -> Result<(Self, usize), Self::Error> {
        let (raw, pos) = RawRecord::decode_record_from(bytes)?;

        if payload::UriPayload::decodes(&raw) {
            let payload = payload::UriPayload::decode_payload(&raw)?;
            return Ok((
                Self::Uri(PayloadRecord { tnf: raw.tnf, id: raw.id, ty: raw.ty, payload }),
                pos,
            ));
        }

        Ok((Self::Raw(raw), pos))
    }
}
