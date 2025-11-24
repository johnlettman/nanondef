#[cfg(any(feature = "std", feature = "alloc"))]
pub use alloc::vec::Vec;

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for [u8] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl AsBytes for &[u8] {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

impl AsBytes for str {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl AsBytes for Vec<u8> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const L: usize> AsBytes for heapless::Vec<u8, L> {
    #[inline]
    fn as_bytes(&self) -> &[u8] {
        self.as_slice()
    }
}

#[inline(always)]
pub(crate) fn encode_u32_be(n: u32, out: &mut [u8]) {
    debug_assert!(out.len() >= 4);

    out[0] = (n >> 24) as u8;
    out[1] = (n >> 16) as u8;
    out[2] = (n >> 8) as u8;
    out[3] = n as u8;
}

#[inline(always)]
pub(crate) fn decode_u32_be(bytes: &[u8]) -> u32 {
    debug_assert!(bytes.len() >= 4);

    ((bytes[0] as u32) << 24)
        | ((bytes[1] as u32) << 16)
        | ((bytes[2] as u32) << 8)
        | (bytes[3] as u32)
}
