mod block;
mod block_tag;
mod capability_container;
mod features;
mod heapless_tag;
mod iter_block;
pub mod message;
mod raw_tag;
mod version;

pub use block::*;
pub use block_tag::*;
pub use capability_container::*;
pub use features::*;
pub use heapless_tag::*;
pub use iter_block::*;
pub use raw_tag::*;
pub use version::*;
use crate::tag::message::DecodeMessage;

#[cfg(any(feature = "std", feature = "alloc"))]
mod tag;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use tag::*;

#[cfg(not(feature = "_ffi_cbindgen"))]
/// Trait for types that can provide an iterator over TLV blocks.
pub trait TagBlocks<'t> {
    fn blocks<M: DecodeMessage<'t>>(&self) -> IterBlock<'t, M>;
}
