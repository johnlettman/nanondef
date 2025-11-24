use crate::{
    error::Truncated,
    tag::message::record::{payload::DecodePayload, Flags, Header, Tnf},
    validate::Validate,
    Encode, Error,
    Error::{
        BufferTooSmall, PayloadTruncated, RecordTnfInvalid, RecordTyInvalidForPayload,
        UriPayloadPrefixInvalid, UriPayloadPrefixNotMatched, UriPayloadUtf8Error,
    },
    OffsetResult,
};

/// An encodable NFC URI Record (RTD-URI).
///
/// This payload corresponds to an NDEF Well-Known Type `"U"` record and uses
/// the NFC Forum URI Identifier Code table to compress common URI prefixes.
///
/// The URI is stored split into:
/// - A **static prefix** chosen from [`PREFIX_MAP`]
/// - A **borrowed suffix** taken directly from the NDEF payload bytes
///
/// [`PREFIX_MAP`]: UriPayload::PREFIX_MAP
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UriPayload<'p> {
    /// URI prefix resolved from the NFC prefix code.
    ///
    /// Stored as a `'static` string taken from
    /// [`PREFIX_MAP`][UriPayload::PREFIX_MAP].
    pub prefix: &'static str,

    /// The URI suffix (remaining text bytes after the prefix).
    pub uri: &'p str,
}

impl<'p> UriPayload<'p> {
    /// Creates a new [`UriPayload`] by inspecting a full URI string and
    /// selecting the **longest matching NFC prefix** from the [`PREFIX_MAP`].
    ///
    /// If no prefix matches, this returns a payload with an empty prefix.
    ///
    /// ```rust
    /// use nanondef::message::record::payload::UriPayload;
    ///
    /// let p = UriPayload::from_uri("https://www.example.com/");
    /// assert_eq!(p.prefix, "https://www.");
    /// assert_eq!(p.uri, "example.com/");
    /// ```
    ///
    /// [`PREFIX_MAP`]: UriPayload::PREFIX_MAP
    pub fn from_uri(uri: &'p str) -> Self {
        if let Some((prefix_code, uri)) = Self::encode_to_parts(uri) {
            Self { prefix: Self::PREFIX_MAP[prefix_code as usize], uri }
        } else {
            Self { prefix: "", uri }
        }
    }

    /// Returns the prefix code for a given prefix string.
    ///
    /// This performs an exact match within
    /// [`PREFIX_MAP`][UriPayload::PREFIX_MAP].
    #[inline]
    pub fn prefix_code(prefix: &str) -> Option<u8> {
        Self::PREFIX_MAP.iter().position(|&p| p == prefix).map(|i| i as u8)
    }

    /// Determines the **longest matching NFC URI prefix** in [`PREFIX_MAP`]
    /// and returns the prefix code along with the remaining URI suffix.
    ///
    /// Returns `None` if the string matches no known NFC URI prefixes.
    ///
    /// This function is used by [`from_uri`].
    ///
    /// [`PREFIX_MAP`]: UriPayload::PREFIX_MAP
    /// [`from_uri`]: UriPayload::from_uri
    #[inline]
    pub fn encode_to_parts(input: &str) -> Option<(u8, &str)> {
        let mut best: Option<(usize, &str)> = None;

        for (index, p) in Self::PREFIX_MAP.iter().enumerate() {
            if input.starts_with(p) {
                match best {
                    None => {
                        best = Some((index, *p));
                    },
                    Some((_, best_p)) => {
                        if p.len() > best_p.len() {
                            best = Some((index, *p));
                        }
                    },
                }
            }
        }

        let (index, p) = best?;
        let remainder = &input[p.len()..];
        Some((index as u8, remainder))
    }

    /// Validates that the given [`RawRecord`] contains a well-formed URI
    /// payload.
    ///
    /// Checks:
    /// - TNF = `WellKnown`
    /// - Type = `"U"`
    /// - Payload is non-empty
    ///
    /// Used by both [`decodes`][DecodePayload::decodes] and
    /// [`decode_payload`][DecodePayload::decode_payload].
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    fn check(flags: &Flags, header: &Header) -> crate::Result<()> {
        if flags.tnf != Tnf::WellKnown {
            return Err(RecordTnfInvalid { got: flags.tnf, want: Tnf::WellKnown });
        }

        if header.ty != "U" {
            return Err(RecordTyInvalidForPayload("UriPayload"));
        }

        Ok(())
    }
}

impl<'p> core::fmt::Display for UriPayload<'p> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.prefix, self.uri)
    }
}

impl<'p> Validate for UriPayload<'p> {
    type Error = Error;

    fn error(&self) -> Option<Self::Error> {
        (!Self::PREFIX_MAP.contains(&self.prefix))
            .then_some(UriPayloadPrefixNotMatched(self.prefix))
    }
}

impl<'p> Encode for UriPayload<'p> {
    fn encoded_len(&self) -> usize {
        1 + self.uri.len()
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let got = buf.len();
        let want = self.encoded_len();
        if got < want {
            return Err(BufferTooSmall { got, want });
        }

        buf[0] = Self::prefix_code(self.prefix).unwrap_or(0);
        buf[1..].copy_from_slice(self.uri.as_bytes());
        Ok(want)
    }
}

impl<'p> DecodePayload<'p> for UriPayload<'p> {
    type Error = crate::Error;

    #[inline]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    fn decodes_record(flags: &Flags, header: &Header) -> bool {
        Self::check(flags, header).is_ok()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    fn decode_payload(
        bytes: &'p [u8],
        flags: &Flags,
        header: &Header,
    ) -> Result<Self, Self::Error> {
        let bytes_len = bytes.len();

        if bytes_len == 0 {
            return Err(PayloadTruncated(Truncated { got: bytes_len, want: 1 }));
        }

        Self::check(flags, header)?;

        let prefix_code = bytes[0] as usize;
        let prefix =
            Self::PREFIX_MAP.get(prefix_code).ok_or(UriPayloadPrefixInvalid(prefix_code))?;

        let uri = core::str::from_utf8(&bytes[1..]).map_err(UriPayloadUtf8Error)?;

        Ok(Self { prefix, uri })
    }
}
