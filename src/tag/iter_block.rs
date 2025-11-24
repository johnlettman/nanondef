use crate::{
    error::Truncated, Error, Error::BlockTruncated, Range,
};
use crate::tag::{message::DecodeMessage, Block, BlockTag};
use core::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct IterBlock<'b, M: DecodeMessage<'b>> {
    bytes: &'b [u8],
    pos: usize,
    _marker: PhantomData<M>,
}

impl<'b, M: DecodeMessage<'b>> IterBlock<'b, M> {
    #[inline]
    pub fn new(bytes: &'b [u8]) -> Self {
        Self { bytes, pos: 0, _marker: PhantomData }
    }

    #[inline(always)]
    fn read_len(&mut self) -> crate::Result<usize> {
        let len = self.bytes.len();
        if self.pos >= len {
            return Err(BlockTruncated(Truncated { got: len, want: self.pos + 1 }));
        }

        let first = self.bytes[self.pos];
        self.pos += 1;

        if first != 0xFF {
            return Ok(first as usize);
        }

        if self.pos + 1 >= len {
            return Err(BlockTruncated(Truncated { got: len, want: self.pos + 2 }));
        }

        let hi = self.bytes[self.pos] as usize;
        let lo = self.bytes[self.pos + 1] as usize;
        self.pos += 2;

        Ok((hi << 8) | lo)
    }
}

impl<'b, M: DecodeMessage<'b>> Iterator for IterBlock<'b, M> {
    type Item = crate::Result<Block<'b, M>>;

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret, skip(self),))]
    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.bytes;
        let len = bytes.len();

        if self.pos >= len {
            #[cfg(feature = "tracing")]
            tracing::debug!("Reached end of TLV at {} bytes", self.pos);

            return None;
        }

        let tag_byte = bytes[self.pos];
        self.pos += 1;

        let tag = BlockTag::from(tag_byte);

        #[cfg(feature = "tracing")]
        tracing::debug!("New TLV tag: {tag:?}");

        match tag {
            BlockTag::Null => {
                // NULL TLV has no length
                #[cfg(feature = "tracing")]
                tracing::trace!("Matched NULL TLV");
                Some(Ok(Block::Null))
            },
            BlockTag::Terminator => {
                #[cfg(feature = "tracing")]
                tracing::trace!("Matched terminator TLV");
                None
            },

            BlockTag::Message => {
                #[cfg(feature = "tracing")]
                tracing::trace!("Matched message TLV");

                let block_len = match self.read_len() {
                    Ok(l) => l,
                    Err(e) => return Some(Err(e)),
                };

                let block_range = Range::new(self.pos, self.pos + block_len);

                #[cfg(feature = "tracing")]
                tracing::debug!(%block_range, "Message TLV range calculated");

                if block_range.end > len {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Message TLV end exceeds {len} available bytes");

                    return Some(Err(BlockTruncated(Truncated {
                        got: len,
                        want: block_range.end,
                    })));
                }

                match M::decode_message(&bytes[block_range]).map_err(Error::implementation_error) {
                    Ok(message) => Some(Ok(Block::Message(message, block_range))),
                    Err(e) => Some(Err(e)),
                }
            },

            // unknown block
            tag => {
                #[cfg(feature = "tracing")]
                tracing::trace!(?tag, "Matched unknown TLV");

                let block_len = match self.read_len() {
                    Ok(l) => l,
                    Err(e) => return Some(Err(e)),
                };

                let block_range = Range::new(self.pos, self.pos + block_len);

                #[cfg(feature = "tracing")]
                tracing::debug!(%block_range, "Unknown TLV range calculated");

                if block_range.end > len {
                    #[cfg(feature = "tracing")]
                    tracing::warn!("Unknown TLV end exceeds {len} available bytes");

                    return Some(Err(BlockTruncated(Truncated {
                        got: len,
                        want: block_range.end,
                    })));
                }

                Some(Ok(match tag {
                    BlockTag::Proprietary => {
                        Block::Proprietary(&bytes[block_range.as_core()], block_range)
                    },
                    BlockTag::Other(_) => {
                        Block::Other(tag, &bytes[block_range.as_core()], block_range)
                    },
                    _ => unreachable!("shouldn't reach here with unknown tag"),
                }))
            },
        }
    }
}
