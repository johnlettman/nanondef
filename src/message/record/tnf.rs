use crate::message::record::flags;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Tnf {
    /// Indicates no `type`, `id`, or `payload` is associated with the NDEF
    /// Record.
    ///
    /// This record type is useful on newly formatted cards since every NDEF tag
    /// must have at least one NDEF Record.
    Empty = 0x00,

    /// Indicates the type field uses the RTD type name format.
    ///
    /// This type name is used to store any record defined by a Record Type
    /// Definition (RTD), such as storing RTD Text, RTD URIs, etc., and is
    /// one of the mostly frequently used and useful record types.
    WellKnown = 0x01,

    /// Indicates the payload is an intermediate or final chunk of a chunked
    /// NDEF Record.
    MimeMedia = 0x02,

    /// Indicates the type field contains a value that follows the absolute-URI
    /// BNF construct defined by RFC 3986.
    AbsoluteUri = 0x03,

    /// Indicates the type field contains a value that follows the RTD external
    /// name specification.
    External = 0x04,

    /// Indicates the payload type is unknown.
    Unknown = 0x05,

    /// Indicates the payload is an intermediate or final chunk of a chunked
    /// NDEF Record.
    Unchanged = 0x06,
    Reserved = 0x07,
}

impl core::fmt::Display for Tnf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => "empty",
                Self::WellKnown => "well-known",
                Self::MimeMedia => "mime media",
                Self::AbsoluteUri => "absolute-URI",
                Self::External => "external",
                Self::Unknown => "unknown",
                Self::Unchanged => "unchanged",
                Self::Reserved => "reserved",
            }
        )
    }
}

impl From<u8> for Tnf {
    fn from(v: u8) -> Self {
        match v & flags::TNF_MASK {
            0 => Tnf::Empty,
            1 => Tnf::WellKnown,
            2 => Tnf::MimeMedia,
            3 => Tnf::AbsoluteUri,
            4 => Tnf::External,
            5 => Tnf::Unknown,
            6 => Tnf::Unchanged,
            _ => Tnf::Reserved,
        }
    }
}
