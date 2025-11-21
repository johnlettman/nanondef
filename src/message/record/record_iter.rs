use crate::message::record::DecodeRecord;
use core::marker::PhantomData;
use crate::Range;

#[derive(Debug, Clone)]
pub struct RecordIter<'r, R: DecodeRecord<'r>> {
    bytes: &'r [u8],
    pos: usize,
    done: bool,
    _marker: PhantomData<R>,
}

impl<'r, R: DecodeRecord<'r>> RecordIter<'r, R> {
    pub fn new(bytes: &'r [u8]) -> Self {
        Self { bytes, pos: 0, done: false, _marker: PhantomData }
    }
}

impl<'r, R: DecodeRecord<'r>> Iterator for RecordIter<'r, R> {
    type Item = crate::Result<(R, Range)>;

    #[cfg_attr(feature = "tracing",
        tracing::instrument(
            level = "trace",
            ret,
            skip(self),
            fields(
                bytes = %crate::tracing::TruncatedBytes::<10>(self.bytes),
                pos = self.pos,
                done = self.done,
            )
        )
    )]
    fn next(&mut self) -> Option<Self::Item> {
        let len = self.bytes.len();

        if self.done || self.pos >= len {
            return None;
        }

        let record_start = self.pos;
        match R::decode_record_from(&self.bytes[record_start..]) {
            Err(e) => {
                self.done = true;
                Some(Err(crate::Error::record_error::<R>(e)))
            },

            Ok((record, consumed)) => {
                let record_range = Range::new(record_start, record_start + consumed);
                self.pos = record_range.end;
                self.done = record.end();
                Some(Ok((record, record_range)))
            },
        }
    }
}
