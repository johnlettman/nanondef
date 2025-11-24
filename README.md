<div align="center">
    <h1>
    ၊၊||၊ nanondef<br>
    <sub><sup>
        A small, zero-copy, <code>no_std</code>-friendly NDEF parser and encoder
    </sup></sub>
    </h1>

[![crates.io][shield-crates]][link-crates]
[![docs.rs][shield-docsrs]][link-docsrs]

</div>

`nanondef` is a lightweight, allocation-optional library for working with 
**NFC Forum NDEF messages**, 
**Capability Containers (CC)**, 
**TLV blocks**, 
and various record payloads.

It is designed for:
- Embedded systems
- Firmware and NFC-tag readers/writers 
- High-level Rust applications requiring a safe, efficient NDEF stack
- Environments requiring deterministic memory usage (e.g., _heapless_)

The library never allocates unless the `"std"` or `"alloc"` feature is enabled.

---

## Features
- Zero-copy decoding from `&[u8]`
- Optional allocation or heapless
- Stable, minimal API surface
- Capability Container (CC) parsing
- Typed payload decoding (URI, raw bytes, and user-defined payloads)
- NDEF record validation
- TLV block decoding
- Full serde support for all public types
- WASM support
- Companion C FFI bindings via the [`nanondef-sys`](nanondef-sys) crate


## License

[MIT](LICENSE)



[link-crates]: https://crates.io/crates/nanondef
[link-docsrs]: https://docs.rs/nanondef

[shield-crates]: https://img.shields.io/crates/v/nanondef.svg?style=for-the-badge
[shield-docsrs]: https://img.shields.io/docsrs/nanondef.svg?style=for-the-badge
