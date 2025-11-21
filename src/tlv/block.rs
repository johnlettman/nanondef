use crate::message::DecodeMessage;
use crate::tlv::BlockTag;
use crate::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Block<'b, M: DecodeMessage<'b>> {
    Null,
    Message(M, Range),
    Proprietary(&'b [u8], Range),
    Terminator(usize),
    Other(BlockTag, &'b [u8], Range),
}
