use crate::{
    bytes::{decode_u32_be, encode_u32_be},
    error::Truncated,
    tag::message::record::{
        payload::DecodePayload, DecodeRecord, EncodeRecord, Flags, Header, MAX_ID_LEN,
        MAX_TY_LEN, PAYLOAD_SR_THRESHOLD,
    },
    trace,
    validate::Validate,
    CrateConsumed, Encode, Error,
    Error::{
        BufferTooSmall, PayloadInvalid, RecordIdTooLong, RecordTruncated, RecordTyTooLong,
        RecordTyUtf8Error,
    },
};

/// A fully decoded NDEF record consisting of a parsed header and a typed
/// payload.
///
/// This type represents the *logical* form of a record after the raw NDEF
/// header, type, ID, and payload fields have been parsed and validated.
///
/// The lifetime `'r` ties borrowed header fields (e.g., type strings, ID
/// slices) to the underlying message buffer.
///
/// # Type Parameters
/// - `P`: A payload type implementing [`Encode`] and [`DecodePayload`]. This
///   allows strongly typed payloads such as URI, Text, MIME types, or raw bytes
///   (`&[u8]`).
#[derive(Debug, Clone)]
pub struct Data<'r, P: Encode + DecodePayload<'r>> {
    /// The decoded NDEF header.
    pub header: Header<'r>,

    /// The strongly typed payload for this record.
    pub payload: P,
}

impl<'r, P: Encode + DecodePayload<'r>> Validate for Data<'r, P> {
    type Error = Error;

    fn error(&self) -> Option<Self::Error> {
        if let Some(id) = self.header.id {
            let id_len = id.len();
            if id_len > MAX_ID_LEN {
                return Some(RecordIdTooLong(id_len));
            }
        }

        let ty_len = self.header.ty.len();
        if ty_len > MAX_TY_LEN {
            return Some(RecordTyTooLong(ty_len));
        }

        self.payload.error().map(Error::implementation_error)
    }
}

impl<'r, P> Encode for Data<'r, P>
where
    P: Encode + DecodePayload<'r>,
{
    fn encoded_len(&self) -> usize {
        let mut len = 0;

        // Header: always 1 byte
        len += 1;

        // Type Length: always 1 byte
        len += 1;

        // Payload Length
        if self.payload.encoded_len() <= PAYLOAD_SR_THRESHOLD {
            // short record: 1 byte
            len += 1;
        } else {
            // long record: 4 bytes
            len += 4;
        }

        // ID length field (only if IL flag set)
        if let Some(id) = self.header.id {
            // length byte + length
            len += 1;
            len += id.len();
        }

        // Type field
        len += self.header.ty.len();

        // Payload field
        len += self.payload.encoded_len();

        len
    }

    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.encode_record_into(buf, false, false)
    }
}

impl<'r, P> EncodeRecord for Data<'r, P>
where
    P: Encode + DecodePayload<'r>,
{
    fn flags(&self, mb: bool, me: bool) -> Flags {
        Flags {
            mb,
            me,
            cf: false,
            sr: self.payload.encoded_len() <= 255,
            il: self.header.id.is_some(),
            tnf: self.header.tnf,
        }
    }

    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool) -> Result<usize, Self::Error> {
        let mut pos = 0;

        // 1. Encode Flags
        pos += encode_flags(self.flags(mb, me), &mut buf[pos..])?;

        // 2. Encode Type Length
        let ty_len = self.header.ty.len();
        pos += encode_ty_len(&mut buf[pos..], ty_len)?;

        // 3. Encode Payload Length
        pos += encode_payload_len(&mut buf[pos..], &self.payload)?;

        // 4. Encode ID
        pos += encode_id(&mut buf[pos..], self.header.id)?;

        // 5. Encode Type
        pos += encode_ty(&mut buf[pos..], self.header.ty)?;

        // 6. Encode Payload
        pos += encode_payload(&mut buf[pos..], &self.payload)
            .map_err(|e| Error::implementation_error_at(e, pos))?;

        debug_assert_eq!(pos, self.encoded_len());
        Ok(pos)
    }
}

impl<'r, P> DecodeRecord<'r> for Data<'r, P>
where
    P: Encode + DecodePayload<'r>,
{
    type Error = Error;

    fn decode_from(bytes: &'r [u8], flags: Flags, header: Header<'r>) -> Result<Self, Self::Error> {
        if !P::decodes_record(&flags, &header) {
            return Err(PayloadInvalid(core::any::type_name::<P>()));
        }

        let payload =
            P::decode_payload(&bytes, &flags, &header).map_err(Error::implementation_error)?;

        Ok(Self { header, payload })
    }
}

#[cfg(feature = "serde")]
impl<'r, P> serde::Serialize for Data<'r, P>
where
    P: Encode + DecodePayload<'r> + serde::Serialize,
    Header<'r>: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("Data", 2)?;
        s.serialize_field("header", &self.header)?;
        s.serialize_field("payload", &self.payload)?;
        s.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, 'r, P> serde::Deserialize<'de> for Data<'r, P>
where
    P: Encode + DecodePayload<'r> + serde::Deserialize<'de>,
    Header<'r>: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct DataOwned<H, P> {
            header: H,
            payload: P,
        }

        let owned = DataOwned::<Header<'r>, P>::deserialize(deserializer)?;

        Ok(Data { header: owned.header, payload: owned.payload })
    }
}

pub fn encode_flags(header: Flags, buf: &mut [u8]) -> crate::Result<usize> {
    if buf.is_empty() {
        return Err(BufferTooSmall { got: 0, want: 1 });
    }

    buf[0] = header.into();
    Ok(1)
}

pub fn encode_ty_len(buf: &mut [u8], ty_len: usize) -> crate::Result<usize> {
    if buf.is_empty() {
        return Err(BufferTooSmall { got: 0, want: 1 });
    }

    if ty_len > MAX_TY_LEN {
        return Err(RecordTyTooLong(ty_len));
    }

    buf[0] = ty_len as u8;
    Ok(1)
}

pub fn encode_ty(buf: &mut [u8], ty: &str) -> crate::Result<usize> {
    let ty_len = ty.len();
    let buf_len = buf.len();

    if buf_len < ty_len {
        return Err(BufferTooSmall { got: buf_len, want: ty_len });
    }

    buf[..ty_len].copy_from_slice(ty.as_bytes());
    Ok(ty_len)
}

pub fn encode_id_len(buf: &mut [u8], id_len: usize) -> crate::Result<usize> {
    if buf.is_empty() {
        return Err(BufferTooSmall { got: 0, want: 1 });
    }

    if id_len > MAX_ID_LEN {
        return Err(RecordIdTooLong(id_len));
    }

    buf[0] = id_len as u8;
    Ok(1)
}

pub fn encode_id(buf: &mut [u8], id: Option<&[u8]>) -> crate::Result<usize> {
    if let Some(id) = id {
        let id_len = id.len();
        let pos = encode_id_len(buf, id_len)?;
        buf[pos..pos + id_len].copy_from_slice(id);
        return Ok(pos + id_len);
    }
    Ok(0)
}

pub fn encode_payload_len<'p, P: Encode + DecodePayload<'p>>(
    buf: &mut [u8],
    payload: &P,
) -> crate::Result<usize> {
    let payload_len = payload.encoded_len();

    if payload_len <= PAYLOAD_SR_THRESHOLD {
        // Short Record
        if buf.is_empty() {
            return Err(BufferTooSmall { got: 0, want: 1 });
        }

        buf[0] = payload_len as u8;
        Ok(1)
    } else {
        // Long Record
        if buf.len() < 4 {
            return Err(BufferTooSmall { got: buf.len(), want: 4 });
        }

        encode_u32_be(payload_len as u32, &mut buf[..4]);
        Ok(4)
    }
}

pub fn encode_payload<'p, P: Encode + DecodePayload<'p>>(
    buf: &mut [u8],
    payload: &P,
) -> crate::Result<usize> {
    let payload_len = payload.encoded_len();
    payload.encode_into(&mut buf[..payload_len]).map_err(Error::implementation_error)?;
    Ok(payload_len)
}

pub fn decode_flags(buf: &[u8]) -> CrateConsumed<Flags> {
    if buf.is_empty() {
        return Err(RecordTruncated(Truncated { got: 0, want: 1 }));
    }

    let flags = Flags::from(buf[0]);
    Ok((1, flags))
}

pub fn decode_ty_len(buf: &[u8]) -> CrateConsumed<usize> {
    if buf.is_empty() {
        return Err(RecordTruncated(Truncated { got: 0, want: 1 }));
    }

    let ty_len = buf[0] as usize;
    Ok((1, ty_len))
}

pub fn decode_ty(buf: &[u8], ty_len: usize) -> CrateConsumed<&str> {
    let buf_len = buf.len();

    if buf_len < ty_len {
        return Err(RecordTruncated(Truncated { got: buf_len, want: ty_len }));
    }

    let ty = core::str::from_utf8(&buf[0..ty_len]).map_err(RecordTyUtf8Error)?;
    Ok((ty_len, ty))
}

pub fn decode_id_len(buf: &[u8]) -> CrateConsumed<usize> {
    if buf.is_empty() {
        return Err(RecordTruncated(Truncated { got: 0, want: 1 }));
    }

    let id_len = buf[0] as usize;
    Ok((1, id_len))
}

pub fn decode_id(buf: &[u8]) -> CrateConsumed<Option<&[u8]>> {
    let buf_len = buf.len();
    let (pos, id_len) = decode_id_len(buf)?;

    if id_len == 0 {
        return Ok((pos, None));
    }

    let id_end = pos + id_len;
    if buf_len < id_end {
        return Err(RecordTruncated(Truncated { got: buf_len, want: id_end }));
    }

    let id = &buf[pos..id_len];
    Ok((id_end, Some(id)))
}

pub fn decode_payload_len(buf: &[u8], sr: bool) -> CrateConsumed<usize> {
    if sr {
        // Short Record
        if buf.is_empty() {
            return Err(BufferTooSmall { got: 0, want: 1 });
        }

        Ok((1, buf[0] as usize))
    } else {
        // Long Record
        let buf_len = buf.len();

        if buf_len < 4 {
            return Err(BufferTooSmall { got: buf_len, want: 4 });
        }

        Ok((4, decode_u32_be(&buf[..4]) as usize))
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace", ret))]
pub fn decode_from_raw<'b>(bytes: &'b [u8]) -> CrateConsumed<(Data<'b, &'b [u8]>, Flags)> {
    let mut pos = 0;

    // 1. Decode Flags
    let (consumed, flags) = decode_flags(bytes)?;
    pos += consumed;
    trace::trace!(pos, ?flags, "Decoded: Flags");

    // 2. Decode Type Length
    let (consumed, ty_len) = decode_ty_len(&bytes[pos..])?;
    pos += consumed;
    trace::trace!(pos, ty_len, "Decoded: Type Length");

    // 3. Decode Payload Length
    let (consumed, payload_len) = decode_payload_len(&bytes[pos..], flags.sr)?;
    pos += consumed;
    trace::trace!(pos, payload_len, "Decoded: Payload Length");

    // 4. (Optionally) Decode ID
    let (consumed, id) = if flags.il { decode_id(&bytes[pos..])? } else { (0, None) };
    pos += consumed;
    trace::trace!(pos, id, "Decoded: ID");

    // 5. Decode Type
    let (consumed, ty) = decode_ty(&bytes[pos..], ty_len)?;
    pos += consumed;
    trace::trace!(pos, ty, "Decoded: Type");

    // 6. Construct Header
    let header = Header::new(id, flags.tnf, ty);
    trace::trace!(?header, "Constructed: Header");

    // 7. Decode Payload
    let payload_end = pos + payload_len;
    let payload = &bytes[pos..payload_end];
    println!("{bytes:?} {pos} {payload_len} {payload:?}");

    Ok((payload_end, (Data { header, payload }, flags)))
}
