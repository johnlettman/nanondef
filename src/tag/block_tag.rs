use core::fmt::Formatter;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BlockTag {
    /// Block is null and should be ignored.
    Null = 0x00,

    /// Block contains an NDEF message.
    Message = 0x03,

    /// Block contains proprietary information.
    Proprietary = 0xFD,

    /// Last TLV block in the data area.
    Terminator = 0xFE,

    /// Other blocks.
    Other(u8),
}

impl core::fmt::Display for BlockTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Message => write!(f, "message"),
            Self::Proprietary => write!(f, "proprietary"),
            Self::Terminator => write!(f, "terminator"),
            Self::Other(v) => write!(f, "other ({})", v),
        }
    }
}

impl From<u8> for BlockTag {
    fn from(t: u8) -> Self {
        match t {
            0x00 => Self::Null,
            0x03 => Self::Message,
            0xFD => Self::Proprietary,
            0xFE => Self::Terminator,
            _ => Self::Other(t),
        }
    }
}

impl From<BlockTag> for u8 {
    fn from(t: BlockTag) -> Self {
        match t {
            BlockTag::Null => 0x00,
            BlockTag::Message => 0x03,
            BlockTag::Proprietary => 0xFD,
            BlockTag::Terminator => 0xFE,
            BlockTag::Other(v) => v,
        }
    }
}
