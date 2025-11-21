#![cfg_attr(not(feature = "std"), no_std)]

pub mod error;
pub mod message;
mod range;
mod tag;
pub mod tlv;
pub(crate) mod trace;
pub(crate) mod bytes;

pub use error::Error;
pub use range::*;
pub use tag::*;

#[cfg(feature = "alloc")]
extern crate alloc;
extern crate core;

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(all(not(feature= "std"), feature = "alloc"))]
pub use alloc::vec::Vec;

#[cfg(feature = "tracing")]
pub trait Encode: Sized + trace::MaybeDebug {
    type Error: core::fmt::Display + trace::MaybeDebug;

    /// Required buffer size for the object.
    fn encoded_len(&self) -> usize;

    /// Encode the object into the given buffer.
    fn encode_into(&self, buf: &mut [u8]) -> core::result::Result<usize, Self::Error>;

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode(&self) -> core::result::Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode_into(&mut buf)?;
        Ok(buf)
    }

    #[cfg(not(any(feature = "std", feature = "alloc")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode<const L: usize>(&self) -> core::result::Result<heapless::Vec<u8, L>, Self::Error> {
        if L < self.encoded_len() {
            panic!("undersized encode Vec buffer");
        }

        let mut buf = heapless::Vec::new();
        self.encode_into(&mut buf)?;
        Ok(buf)
    }
}



