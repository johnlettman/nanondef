use crate::{
    tag::message::record::{
        payload, payload::DecodePayload, Data, DecodeRecord, EncodeRecord, Flags, Header,
    },
    validate::Validate,
    Encode, Error,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum Record<'r> {
    /// A record whose payload is raw bytes.
    Raw(Data<'r, &'r [u8]>),

    /// A record whose payload is a parsed RTD-URI value.
    Uri(Data<'r, payload::UriPayload<'r>>),
}

impl<'r> Record<'r> {}

impl<'r> Validate for Record<'r> {
    type Error = Error;

    fn error(&self) -> Option<Self::Error> {
        match self {
            Self::Raw(raw) => raw.error(),
            Self::Uri(uri) => uri.error(),
        }
    }
}

impl<'r> Encode for Record<'r> {
    #[inline]
    fn encoded_len(&self) -> usize {
        match self {
            Self::Raw(raw) => raw.encoded_len(),
            Self::Uri(uri) => uri.encoded_len(),
        }
    }

    #[inline(always)]
    fn encode_into(&self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.encode_record_into(buf, false, false)
    }
}

impl<'r> EncodeRecord for Record<'r> {
    fn flags(&self, mb: bool, me: bool) -> Flags {
        match self {
            Self::Raw(raw) => raw.flags(mb, me),
            Self::Uri(uri) => uri.flags(mb, me),
        }
    }

    fn encode_record_into(&self, buf: &mut [u8], mb: bool, me: bool) -> Result<usize, Self::Error> {
        match self {
            Self::Raw(raw) => raw.encode_record_into(buf, mb, me),
            Self::Uri(uri) => uri.encode_record_into(buf, mb, me),
        }
    }
}

impl<'r> DecodeRecord<'r> for Record<'r> {
    type Error = Error;

    fn decode_from(bytes: &'r [u8], flags: Flags, header: Header<'r>) -> Result<Self, Self::Error> {
        if payload::UriPayload::decodes_record(&flags, &header) {
            let payload = payload::UriPayload::decode_payload(bytes, &flags, &header)?;
            return Ok(Self::Uri(Data { header, payload }));
        }

        Ok(Self::Raw(Data { header, payload: bytes }))
    }
}
