use crate::{
    Error,
    Error::{BadField, MissingField},
};
use crate::Error::VersionTooLarge;
use crate::validate::Validate;

/// A major/minor semantic version descriptor.
///
/// Used throughout the crate for representing NDEF and CC version fields.
///
/// # Examples
/// ```rust
/// let v = Version::new(1, 3);
/// assert_eq!(v.major(), 1);
/// assert_eq!(v.minor(), 3);
/// ```
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_new::new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Version {
    /// Major version.
    pub major: u8,

    /// Minor version.
    pub minor: u8,
}

impl Version {
    pub const MAJOR_MAX: u8 = 0xF;
    pub const MINOR_MAX: u8 = 0xF;


    /// Encodes the version into a single byte (major << 4 | minor).
    #[inline(always)]
    pub const fn encode(&self) -> u8 {
        (self.major << 4) | (self.minor & 0x0F)
    }

    /// Decodes a CC-style version byte into a Version struct.
    #[inline(always)]
    pub const fn decode(byte: u8) -> Self {
        Self { major: byte >> 4, minor: byte & 0x0F }
    }
}

impl Validate for Version {
    type Error = Error;

    fn error(&self) -> Option<Self::Error> {
        if self.major > Self::MAJOR_MAX {
            Some(VersionTooLarge { field: "major", got: self.major })
        } else if self.minor > Self::MINOR_MAX {
            Some(VersionTooLarge { field: "minor", got: self.minor })
        } else {
            None
        }
    }
}

impl core::str::FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');

        let major =
            parts.next().ok_or(MissingField("major"))?.parse().map_err(|_| BadField("major"))?;

        let minor =
            parts.next().ok_or(MissingField("minor"))?.parse().map_err(|_| BadField("minor"))?;

        Ok(Self::new(major, minor))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        match self.major.cmp(&other.major) {
            core::cmp::Ordering::Equal => self.minor.cmp(&other.minor),
            ord => ord,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl From<(u8, u8)> for Version {
    #[inline(always)]
    fn from((major, minor): (u8, u8)) -> Self {
        Self::new(major, minor)
    }
}

impl From<Version> for (u8, u8) {
    #[inline(always)]
    fn from(v: Version) -> Self {
        (v.major, v.minor)
    }
}

impl From<u8> for Version {
    #[inline(always)]
    fn from(byte: u8) -> Self {
        Version::decode(byte)
    }
}

impl From<Version> for u8 {
    #[inline(always)]
    fn from(v: Version) -> u8 {
        v.encode()
    }
}
