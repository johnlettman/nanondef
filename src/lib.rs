//! # nanondef
//!
//! **nanondef** is a compact, `no_std`-friendly NFC Forum **NDEF encoding and
//! decoding library**, designed for embedded systems, WASM runtimes, and
//! efficient tag-parsing pipelines.
//!
//! The crate provides:
//! - Fully-typed decoding of **NDEF Messages**, **Records**, and **Payloads**
//! - Zero-copy parsing via lifetimes (`&[u8]`)
//! - Pluggable `DecodePayload` trait for supporting custom payload types
//! - Compact encoding via the `Encode` trait (with [`heapless::Vec`] support)
//! - Strongly typed NFC **Capability Container (CC)** components:
//!   - [`CapabilityContainer`]
//!   - [`Version`] (nibble-packed major/minor)
//!   - [`Features`] (bitflag wrapper)
//! - Optional `serde` support for serialization
//! - Optional `wasm_bindgen` bindings
//! - Works in `no_std`, `alloc`, or full `std` environments
//!
//! The library aims to be:
//! - **Correct** – adheres to NFC Forum Type 2 / NDEF encoding rules
//! - **Minimal** – no dependencies by default (only `heapless` optionally)
//! - **Zero-copy** – payload fields borrow directly from the input buffer
//! - **Flexible** – users can define custom payload decoders
//!
//! ## Getting Started
//! ## Decode a full NDEF message from a byte slice
//! ```rust
//! use nanondef::message::{
//!     record::{payload::UriPayload, Record, Tnf},
//!     DecodeMessage, HeaplessMessage, MessageRecords,
//! };
//!
//! // A single-record NDEF message containing a URI payload
//! let bytes: &[u8] = &[
//!     0b1101_0001, // MB | ME | SR | TNF=WellKnown
//!     0x01,        // Type length = 1 ("U")
//!     0x0B,        // Payload length
//!     b'U',        // Type field
//!     0x03,        // Prefix code: "http://"
//!     b'g',
//!     b'i',
//!     b't',
//!     b'h',
//!     b'u',
//!     b'b',
//!     b'.',
//!     b'c',
//!     b'o',
//!     b'm',
//! ];
//!
//! // Decode the message
//! let msg = HeaplessMessage::<1>::decode_message(bytes).expect("should decode message");
//!
//! // Extract the payload as a typed URI payload
//! let records = msg.records().expect("should provide records");
//! let Record::Uri(uri) = &records[0] else { panic!("should be a URI record") };
//!
//! assert_eq!(uri.header.tnf, Tnf::WellKnown);
//! assert_eq!(uri.payload.prefix, "http://");
//! assert_eq!(uri.payload.uri, "github.com");
//! ```
#![cfg_attr(any(docsrs, nightly_doc), feature(doc_cfg, rustdoc_internals))]
#![cfg_attr(any(docsrs, nightly_doc), allow(internal_features))]
#![cfg_attr(not(feature = "std"), no_std)]

mod bytes;
mod dec;
mod enc;
pub mod error;
mod range;
pub mod tag;
pub(crate) mod trace;
mod validate;
pub mod ffi;

pub use bytes::*;
pub use dec::*;
pub use enc::*;
pub use error::Error;
pub use range::*;

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;
extern crate core;

pub type Result<T> = core::result::Result<T, Error>;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
pub use alloc::vec::Vec;
