use crate::{message::record::{payload::DecodePayload, RawRecord, Tnf}, Encode};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UriPayload<'p> {
    /// URI prefix resolved from the NFC prefix code.
    pub prefix: &'static str,

    /// URI text bytes (after prefix).
    pub uri: &'p str,
}

impl<'p> UriPayload<'p> {
    /// URI Identifier Codes (NFC RTD-URI).
    pub const PREFIX_MAP: &'static [&'static str] = &[
        "",                           // 0x00
        "http://www.",                // 0x01
        "https://www.",               // 0x02
        "http://",                    // 0x03
        "https://",                   // 0x04
        "tel:",                       // 0x05
        "mailto:",                    // 0x06
        "ftp://anonymous:anonymous@", // 0x07
        "ftp://ftp.",                 // 0x08
        "ftps://",                    // 0x09
        "sftp://",                    // 0x0A
        "smb://",                     // 0x0B
        "nfs://",                     // 0x0C
        "ftp://",                     // 0x0D
        "dav://",                     // 0x0E
        "news:",                      // 0x0F
        "telnet://",                  // 0x10
        "imap:",                      // 0x11
        "rtsp://",                    // 0x12
        "urn:",                       // 0x13
        "pop:",                       // 0x14
        "sip:",                       // 0x15
        "sips:",                      // 0x16
        "tftp:",                      // 0x17
        "btspp://",                   // 0x18
        "btl2cap://",                 // 0x19
        "btgoep://",                  // 0x1A
        "tcpobex://",                 // 0x1B
        "irdaobex://",                // 0x1C
        "file://",                    // 0x1D
        "urn:epc:id:",                // 0x1E
        "urn:epc:tag:",               // 0x1F
        "urn:epc:pat:",               // 0x20
        "urn:epc:raw:",               // 0x21
        "urn:epc:",                   // 0x22
        "urn:nfc:",                   // 0x23
    ];

    pub fn from_uri(uri: &'p str) -> Self {
        if let Some((prefix_code, uri)) = Self::encode_prefix(uri) {
            Self { prefix: Self::PREFIX_MAP[prefix_code as usize], uri }
        } else {
            Self { prefix: "", uri }
        }
    }

    #[inline]
    pub fn prefix_code(prefix: &str) -> Option<u8> {
        Self::PREFIX_MAP
            .iter()
            .position(|&p| p == prefix)
            .map(|i| i as u8)
    }

    #[inline]
    pub fn encode_prefix(input: &str) -> Option<(u8, &str)> {
        let mut best: Option<(usize, &str)> = None;

        for (i, p) in Self::PREFIX_MAP.iter().enumerate() {
            if input.starts_with(p) {
                match best {
                    None => {
                        best = Some((i, *p));
                    }
                    Some((_, best_p)) => {
                        if p.len() > best_p.len() {
                            best = Some((i, *p));
                        }
                    }
                }
            }
        }

        let (idx, p) = best?;
        let remainder = &input[p.len()..];

        Some((idx as u8, remainder))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    fn check_record(record: &RawRecord<'p>) -> crate::Result<()> {
        if record.tnf != Tnf::WellKnown {
            return Err(crate::Error::InvalidTnf { tnf: record.tnf, want: Tnf::WellKnown });
        }

        if record.ty != "U" {
            return Err(crate::Error::InvalidType("UriPayload"));
        }

        if record.bytes.is_empty() {
            return Err(crate::Error::payload_truncated(record.bytes.len(), 1));
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

impl<'p> Encode for UriPayload<'p> {
    type Error = crate::Error;

    fn encoded_len(&self) -> usize {
        1 + self.uri.len()
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let len = buf.len();
        let want = self.encoded_len();
        if len < want {
            return Err(crate::Error::BufferTooSmall { len, want });
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
    fn decodes(record: &RawRecord<'p>) -> bool {
        Self::check_record(record).is_ok()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
    fn decode_payload(record: &RawRecord<'p>) -> Result<Self, Self::Error> {
        Self::check_record(record)?;

        let prefix_code = record.bytes[0] as usize;
        let prefix =
            Self::PREFIX_MAP.get(prefix_code).ok_or(crate::Error::InvalidUriPrefix(prefix_code))?;

        let uri = core::str::from_utf8(&record.bytes[1..]).map_err(crate::Error::UriUtf8Error)?;

        Ok(Self { prefix, uri })
    }
}
