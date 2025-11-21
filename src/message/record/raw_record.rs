use crate::{message::record::{payload::DecodePayload, DecodeRecord, Header}, Encode, Range};
use crate::bytes::encode_u32_be;
use crate::message::record::{EncodeRecord, Tnf};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RawRecord<'r> {
    pub id: Option<&'r [u8]>,
    pub ty: &'r str,
    pub tnf: Tnf,
    pub bytes: &'r [u8],
}

impl<'r> RawRecord<'r> {
    #[cfg_attr(feature = "tracing",
        tracing::instrument(
            level = "trace",
            ret,
            skip(self)
        )
    )]
    pub fn payload<P: DecodePayload<'r>>(&'r self) -> Result<P, P::Error> {
        P::decode_payload(self)
    }
}

impl<'r> Encode for RawRecord<'r> {
    type Error = crate::Error;

    fn encoded_len(&self) -> usize {
        let mut len = 0;

        // Header: always 1 byte
        len += 1;

        // Type Length: always 1 byte
        len += 1;

        // Payload Length
        if self.bytes.len() <= 255 {
            // short record: 1 byte
            len += 1;
        } else {
            // long record: 4 bytes
            len += 4;
        }

        // ID length field (only if IL flag set)
        if let Some(id) = self.id {
            // length byte + length
            len += 1;
            len += id.len();
        }

        // Type field
        len += self.ty.len();

        // Payload field
        len += self.bytes.len();

        len
    }

    #[inline(always)]
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.encode_record_into(buf, false, false)
    }
}

impl<'r> EncodeRecord for RawRecord<'r> {
    fn header(&self, mb: bool, me: bool) -> Header {
        Header {
            mb,
            me,
            cf: false,
            sr: self.bytes.len() <= 255,
            il: self.id.is_some(),
            tnf: self.tnf,
        }
    }

    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool) -> Result<usize, Self::Error> {
        // 0. Setup and check buf
        // ----------------------
        let want = self.encoded_len();
        if buf.len() < want {
            return Err(crate::Error::BufferTooSmall { want, len: buf.len() })
        }

        let mut pos = 0;

        // 1. Encode Header
        // ----------------
        buf[pos] = self.header(mb, me).into();
        pos += 1;

        // 2. Encode Type Length
        // ---------------------
        let ty_len = self.ty.len();
        if ty_len > 255 {
            return Err(crate::Error::RecordTypeTooLong(ty_len));
        }

        buf[pos] = ty_len as u8;
        pos += 1;

        // 3. Encode Payload Length
        // ------------------------
        let bytes_len = self.bytes.len();
        if bytes_len <= 255 {
            // short record (SR)
            buf[pos] = bytes_len as u8;
            pos += 1;
        } else {
            // long record
            encode_u32_be(bytes_len as u32, &mut buf[pos..pos+3]);
            pos += 4;
        }

        // 4. Encode ID
        // ------------
        if let Some(id) = self.id {
            let id_len = id.len();
            if id_len > 255 {
                return Err(crate::Error::RecordIdTooLong(id_len));
            }

            // ID length
            buf[pos] = id_len as u8;
            pos += 1;

            // ID
            buf[pos .. pos+id_len].copy_from_slice(id);
            pos += id_len;
        }

        // 5. Encode Type
        // --------------
        buf[pos .. pos + ty_len].copy_from_slice(self.ty.as_bytes());
        pos += ty_len;

        // 6. Encode Payload
        // -----------------
        buf[pos .. pos + bytes_len].copy_from_slice(self.bytes);
        pos += bytes_len;

        debug_assert_eq!(pos, want);
        Ok(pos)
    }
}

impl<'r> DecodeRecord<'r> for RawRecord<'r> {
    type Error = crate::Error;


    #[cfg_attr(feature = "tracing",
        tracing::instrument(
            level = "trace",
            ret,
            skip(bytes),
            fields(bytes = %crate::tracing::TruncatedBytes::<10>(bytes))
        )
    )]
    fn decode_record_from(bytes: &'r [u8]) -> Result<(Self, usize), Self::Error> {
        if bytes.is_empty() {
            return Err(crate::Error::EmptyRecord);
        }

        let len = bytes.len();

        let header = Header::from(bytes[0]);
        #[cfg(feature = "tracing")]
        tracing::trace!(?header, "Decoded header");

        let ty_len = bytes[1] as usize;

        // decode Payload Length Field
        let (payload_len, mut pos) = if header.sr {
            // short record
            if len < 3 {
                return Err(crate::Error::record_truncated(len, 3));
            }

            (bytes[2] as usize, 3)
        } else {
            // long record
            if len < 6 {
                return Err(crate::Error::record_truncated(len, 6));
            }

            let payload_len = (bytes[2] as usize) << 24
                | (bytes[3] as usize) << 16
                | (bytes[4] as usize) << 8
                | (bytes[5] as usize);
            (payload_len, 6)
        };
        #[cfg(feature = "tracing")]
        tracing::trace!(payload_len, "Decoded payload length");

        // decode ID Length Field
        let id = if header.il {
            let id_len = *bytes.get(pos).ok_or(crate::Error::record_truncated(len, pos + 1))? as usize;
            let id_range = Range::new(pos, pos + id_len);
            pos += 1;

            #[cfg(feature = "tracing")]
            tracing::trace!(payload_len, "Decoded payload length");

            pos += id_len;
            Some(bytes.get(id_range.as_core()).ok_or(crate::Error::record_truncated(len, pos + id_range.end))?)
        } else {
            None
        };

        // decode Type Field
        let ty_range = Range::new(pos, pos + ty_len);
        #[cfg(feature = "tracing")]
        tracing::trace!(%ty_range, "Calculated type range");

        let ty = core::str::from_utf8(
            bytes
                .get(ty_range.as_core())
                .ok_or(crate::Error::record_truncated(len, ty_range.end))?,
        )
            .map_err(crate::Error::RecordTypeUtf8Error)?;

        #[cfg(feature = "tracing")]
        tracing::trace!(ty, "Decoded record type");
        pos = ty_range.end;

        // decode Payload
        let payload_range = Range::new(pos, pos + payload_len);
        #[cfg(feature = "tracing")]
        tracing::trace!(%payload_range, "Calculated payload range");

        let payload_bytes = bytes
            .get(payload_range.as_core())
            .ok_or(crate::Error::record_truncated(len, payload_range.end))?;

        Ok((RawRecord { id, ty, bytes: payload_bytes, tnf: header.tnf }, payload_range.end))
    }
}
