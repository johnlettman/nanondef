use crate::message::record::RawRecord;

mod uri;

pub use uri::*;
use crate::trace::MaybeDebug;

pub trait DecodePayload<'p>: Sized + MaybeDebug {
    type Error: core::fmt::Display + MaybeDebug;

    #[inline(always)]
    fn decodes(_record: &RawRecord<'p>) -> bool {
        true
    }

    fn decode_payload(record: &RawRecord<'p>) -> Result<Self, Self::Error>;
}
