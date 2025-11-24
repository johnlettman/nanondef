use crate::{trace::MaybeDebug, validate::Validate, AsBytes, Error, Error::BufferTooSmall};

/// A trait for producing an encoded byte representation of a value.
///
/// The [`Encode`] trait defines the low-level mechanism used by NDEF payload
/// types to serialize themselves into a caller-provided buffer.
///
/// This trait supports three forms of encoding, depending on available
/// features:
/// - **no `"std"` and no `"alloc"`**:
///     - [`encode_into`] writes into a caller-owned buffer.
///     - [`encode`] returns a [`heapless::Vec`] with `L` length of `u8`.
///
/// - **`"std"` or `"alloc"` enabled**:
///     - [`encode`] allocates a [`Vec`] with exactly the required size.
///
/// The contract for the encoder is:
/// 1. [`encoded_len`] declares the number of bytes needed to encode `self`.
/// 2. [`encode_into`] writes exactly that many bytes.
/// 3. The caller must supply a buffer *at least* [`encoded_len`] bytes long.
///
/// # Examples
/// ## [`encode`] with the `"std"` or `"alloc"` features
/// ```rust
/// # #[cfg(any(feature = "std", feature = "alloc"))]
/// # {
/// use nanondef::{message::record::payload::UriPayload, Encode};
///
/// let payload = UriPayload::from_uri("https://www.github.com/");
///
/// let bytes = payload.encode()?; // allocates Vec<u8> when std
/// let prefix_code = UriPayload::prefix_code("https://www."); // prefix code for "https://www."
/// assert_eq!(bytes[0], prefix_code);
/// # }
/// ```
///
/// ## [`encode`] without the `"std"` or `"alloc"` features (using [`heapless`])
/// ```rust
/// # #[cfg(not(any(feature = "std", feature = "alloc")))]
/// # {
/// use nanondef::{message::record::payload::UriPayload, Encode};
///
/// let payload = UriPayload::from_uri("https://www.github.com/");
///
/// let bytes: heapless::Vec<u8, 64> = payload.encode()?; // zero allocation
/// let prefix_code = UriPayload::prefix_code("https://www."); // prefix code for "https://www."
/// assert_eq!(bytes[0], prefix_code);
/// # }
/// ```
///
/// ## [`encode_into`]
/// ```rust
/// use nanondef::{message::record::payload::UriPayload, Encode};
///
/// let payload = UriPayload::from_uri("https://www.github.com/");
///
/// let mut buf = [0u8; 64]; // appropriately-sized write buf
/// let used = p.encode_into(&mut buf?); // zero allocation
/// assert_eq!(used, p.encoded_len()); // the consumed amount should equal the provided encode len
/// ```
///
/// # Errors
/// Encoding errors must be reported using the associated [`Error`] type. Many
/// encoders simply return [`BufferTooSmall`]; however, more complex payloads
/// may report specific user-level encoding errors.
///
/// [`encode`]: Encode::encode
/// [`encode_into`]: Encode::encode_into
/// [`encoded_len`]: Encode::encoded_len
/// [`Error`]: Encode::Error
/// [`BufferTooSmall`]: crate::Error::BufferTooSmall
pub trait Encode: Sized + Validate {
    /// Returns the number of bytes required to encode `self`.
    ///
    /// This length must match the number of bytes written by
    /// [`encode_into`][Encode::encode_into].
    fn encoded_len(&self) -> usize;

    /// Encodes `self` into the given buffer, returning the number of bytes
    /// written.
    ///
    /// The caller must ensure that `buf.len() >= encoded_len()`.
    ///
    /// Implementors should **never** write beyond
    /// [`encoded_len`][Encode::encoded_len] bytes.
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error>;

    /// Allocates a buffer and encodes the value into it.
    ///
    /// Available only when the `"std"` or `"alloc"` features are enabled.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode(&self) -> Result<Vec<u8>, Self::Error> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode_into(&mut buf)?;
        Ok(buf)
    }

    /// Encodes into a fixed-capacity [`heapless::Vec`].
    ///
    /// Requires that the caller choose a capacity `L` large enough to hold
    /// [`encoded_len`][Encode::encoded_len] bytes. If the capacity is
    /// insufficient, this function panics.
    ///
    /// Available only in builds without the `"std"` or `"alloc"` feature.
    #[cfg(not(any(feature = "std", feature = "alloc")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    fn encode<const L: usize>(&self) -> Result<heapless::Vec<u8, L>, Self::Error> {
        if L < self.encoded_len() {
            panic!("undersized encode Vec buffer");
        }

        let mut buf = heapless::Vec::new();
        self.encode_into(&mut buf)?;
        Ok(buf)
    }
}

impl<T: AsBytes + MaybeDebug> Validate for T {
    type Error = Error;
}

impl<T: AsBytes + MaybeDebug> Encode for T {
    fn encoded_len(&self) -> usize {
        self.as_bytes().len()
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let buf_len = buf.len();
        let enc_len = self.encoded_len();

        if buf_len < enc_len {
            return Err(BufferTooSmall { got: buf_len, want: enc_len });
        }

        let as_ref = self.as_bytes();
        buf.copy_from_slice(&as_ref[0..enc_len]);
        Ok(enc_len)
    }
}
