use crate::tag::message::record::Tnf;

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flags {
    /// **Message Begin** indicates if this is the start of an NDEF message.
    pub mb: bool,

    /// **Message End** indicates if this is the last record in the message.
    pub me: bool,

    /// **Chunk Flag** indicates if this is the first record chunk or a middle
    /// record chunk.
    pub cf: bool,

    /// **Short Record** is set to `true` if the payload length is 1 byte or
    /// less, allowing for more compact records.
    pub sr: bool,

    /// **ID Length** indicates if the _ID Length_ field is present.
    pub il: bool,

    /// **Type Name Format** describes the record type and sets the expectation
    /// for the structure and content for the rest of the record.
    pub tnf: Tnf,
}

impl Flags {
    /// Message Begin.
    pub const MB: u8 = 1 << 7;

    /// Message End.
    pub const ME: u8 = 1 << 6;

    /// Chunk Flag.
    pub const CF: u8 = 1 << 5;

    /// Short Record.
    pub const SR: u8 = 1 << 4;

    /// ID Length.
    pub const IL: u8 = 1 << 3;

    pub const TNF_MASK: u8 = 0b111;
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Self {
            mb: (byte & Self::MB) != 0,
            me: (byte & Self::ME) != 0,
            cf: (byte & Self::CF) != 0,
            sr: (byte & Self::SR) != 0,
            il: (byte & Self::IL) != 0,
            tnf: Tnf::from(byte & Self::TNF_MASK),
        }
    }
}

impl From<Flags> for u8 {
    #[inline(always)]
    fn from(h: Flags) -> u8 {
        let mut byte = 0u8;

        if h.mb {
            byte |= Flags::MB;
        }
        if h.me {
            byte |= Flags::ME;
        }
        if h.cf {
            byte |= Flags::CF;
        }
        if h.sr {
            byte |= Flags::SR;
        }
        if h.il {
            byte |= Flags::IL;
        }

        byte |= (h.tnf as u8) & Flags::TNF_MASK;

        byte
    }
}
