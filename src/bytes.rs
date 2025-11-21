#[inline(always)]
pub fn encode_u32_be(n: u32, out: &mut [u8]) {
    debug_assert!(out.len() >= 4);

    out[0] = (n >> 24) as u8;
    out[1] = (n >> 16) as u8;
    out[2] = (n >> 8)  as u8;
    out[3] = n as u8;
}

#[inline(always)]
pub fn decode_u32_be(bytes: &[u8]) -> u32 {
    debug_assert!(bytes.len() >= 4);

    ((bytes[0] as u32) << 24) |
        ((bytes[1] as u32) << 16) |
        ((bytes[2] as u32) << 8)  |
        (bytes[3] as u32)
}
