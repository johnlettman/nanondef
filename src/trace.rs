


pub struct TruncatedBytes<'a, const L: usize>(pub &'a [u8]);

impl<'a, const L: usize> core::fmt::Display for TruncatedBytes<'a, L> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.0;
        let len = bytes.len();
        let display_len = bytes.iter().take(L);

        write!(f, "[")?;

        for b in display_len {
            write!(f, "0x{:02x}, ", b)?;
        }

        if len > L {
            write!(f, "… ({} more)", len - L)?;
        }

        write!(f, "]")
    }
}

#[cfg(feature = "tracing")]
pub trait MaybeDebug: core::fmt::Debug {}

#[cfg(not(feature = "tracing"))]
pub trait MaybeDebug {}

#[cfg(feature = "tracing")]
impl<T: core::fmt::Debug> MaybeDebug for T {}

#[cfg(not(feature = "tracing"))]
impl<T> MaybeDebug for T {}

#[cfg(feature = "tracing")]
pub(crate) use tracing::{debug, error, info, trace, warn};

#[cfg(not(feature = "tracing"))]
macro_rules! log_debug {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_trace {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_info {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_warn {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_error {
    ($($t:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
pub(crate) use {
    log_debug as debug, log_error as error, log_info as info, log_trace as trace, log_warn as warn,
};
