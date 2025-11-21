use crate::bytes::encode_u32_be;
use crate::Encode;
use crate::message::record::{payload::DecodePayload, DecodeRecord, EncodeRecord, Header, RawRecord, Tnf};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PayloadRecord<'r, P: Encode + DecodePayload<'r>> {
    pub id: Option<&'r [u8]>,
    pub tnf: Tnf,
    pub ty: &'r str,
    pub payload: P,
}

impl<'r, P> Encode for PayloadRecord<'r, P>
where
    P: Encode + DecodePayload<'r>
{
    type Error = crate::Error;

    fn encoded_len(&self) -> usize {
        let mut len = 0;

        // Header: always 1 byte
        len += 1;

        // Type Length: always 1 byte
        len += 1;

        // Payload Length
        if self.payload.encoded_len() <= 255 {
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
        len += self.payload.encoded_len();

        len
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.encode_record_into(buf, false, false)
    }
}

impl<'r, P> EncodeRecord for PayloadRecord<'r, P>
where
    P: Encode + DecodePayload<'r>
{
    fn header(&self, mb: bool, me: bool) -> Header {
        Header {
            mb,
            me,
            cf: false,
            sr: self.payload.encoded_len() <= 255,
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
        let payload_len = self.payload.encoded_len();
        if payload_len <= 255 {
            // short record (SR)
            buf[pos] = payload_len as u8;
            pos += 1;
        } else {
            // long record
            encode_u32_be(payload_len as u32, &mut buf[pos..pos+3]);
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
        self.payload.encode_into(&mut buf[pos..pos + payload_len]).map_err(crate::Error::payload_error::<P>)?;
        pos += payload_len;

        debug_assert_eq!(pos, want);
        Ok(pos)
    }
}

impl<'r, P> DecodeRecord<'r> for PayloadRecord<'r, P>
where
    P: Encode + DecodePayload<'r>,
{
    type Error = crate::Error;

    fn decode_record_from(bytes: &'r [u8]) -> Result<(Self, usize), Self::Error> {
        let (record, pos) = RawRecord::decode_record_from(bytes)?;

        if !P::decodes(&record) {
            return Err(crate::Error::InvalidPayload(core::any::type_name::<P>()));
        }

        let payload = P::decode_payload(&record).map_err(crate::Error::payload_error::<P>)?;

        Ok((Self { header: record.header, ty: record.ty, payload }, pos))
    }
}
