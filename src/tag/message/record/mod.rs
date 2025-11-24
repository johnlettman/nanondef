mod data;
mod flags;
mod header;
pub mod payload;
mod record;
mod tnf;
mod util;

use crate::{
    error::ImplementationError, trace::MaybeDebug, validate::Validate, ConsumedResult, Encode,
    OffsetResult,
};
pub use data::*;
pub use flags::*;
pub use header::*;
pub use record::*;
pub use tnf::*;
pub use util::*;

pub trait EncodeRecord: Sized + Encode + Validate {
    fn flags(&self, mb: bool, me: bool) -> Flags;

    /// Encode the record into the given buffer.
    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool)
        -> ConsumedResult<Self::Error>;

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode_record(&self, mb: bool, me: bool) -> OffsetResult<Vec<u8>, Self::Error> {
        let len = self.encoded_len();
        let mut buf = Vec::with_capacity(len);
        self.encode_record_into(&mut buf, mb, me)?;
        Ok((len, buf))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode_record_heapless<const L: usize>(
        &self,
        mb: bool,
        me: bool,
    ) -> OffsetResult<heapless::Vec<u8, L>, Self::Error> {
        if L < self.encoded_len() {
            panic!("undersized encode Vec buffer");
        }

        let mut buf = heapless::Vec::new();
        self.encode_record_into(&mut buf, mb, me)?;
        Ok((L, buf))
    }
}

pub trait DecodeRecord<'r>: Sized + MaybeDebug {
    type Error: ImplementationError;

    fn decode_from(bytes: &'r [u8], flags: Flags, header: Header<'r>) -> Result<Self, Self::Error>;

    fn decode_from_raw(bytes: &'r [u8]) -> OffsetResult<Self, Self::Error> {
        let (pos, (raw, flags)) = decode_from_raw(bytes).map_err(Self::Error::custom)?;
        Ok((pos, Self::decode_from(&raw.payload, flags, raw.header)?))
    }
}
