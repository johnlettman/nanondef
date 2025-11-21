use crate::message::record::{DecodeRecord, RecordIter};
use crate::message::DecodeMessage;

#[derive(Debug, Clone)]
pub struct RawMessage<'m>(&'m [u8]);

impl<'m> RawMessage<'m> {
    pub fn records<R: DecodeRecord<'m>>(&self) -> RecordIter<'m, R> {
        RecordIter::<'m, R>::new(self.0)
    }
}

impl<'m> DecodeMessage<'m> for RawMessage<'m> {
    type Error = crate::Error;

    #[inline(always)]
    fn decode_message(bytes: &'m [u8]) -> Result<Self, Self::Error> {
        Ok(Self(bytes))
    }
}
