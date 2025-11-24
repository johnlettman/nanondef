/// A bitfield describing feature flags in an NDEF Capability Container (CC).
///
/// This wraps a single [`u8`] value, where each bit corresponds to a CC
/// capability defined by the NFC Forum specifications.
///
/// The type provides convenience methods as well as bit-manipulation helpers.
///
/// # Features
/// - [`EMPTY`] - No features enabled.
///     - Check whether set: [`empty`]
/// - [`READ_ONLY`] - Read-only NDEF data.
///     - Check whether set: [`read_only`]
/// - [`MULTI_RECORD`] - Supports multi-record NDEF messages.
///     - Check whether set: [`multi_record`]
/// - [`EXTENDED_MEMORY`] - Tag supports extended memory (> 255 bytes).
///    - Check whether set: [`extended_memory`]
/// - [`LOCKABLE`] - Tag supports lock-byte areas.
///    - Check whether set: [`lockable`]
/// - [`DYNAMIC`] - Tag supports dynamic features.
///   - Check whether set: [`dynamic`]
///
/// # Examples
/// ```rust
/// let mut f = Features::EMPTY;
/// f.enable(Features::READ_ONLY);
/// assert!(f.read_only());
/// ```
///
/// [`EMPTY`]: Features::EMPTY
/// [`READ_ONLY`]: Features::READ_ONLY
/// [`MULTI_RECORD`]: Features::MULTI_RECORD
/// [`EXTENDED_MEMORY`]: Features::EXTENDED_MEMORY
/// [`LOCKABLE`]: Features::LOCKABLE
/// [`DYNAMIC`]: Features::DYNAMIC
///
/// [`empty`]: Features::empty
/// [`read_only`]: Features::read_only
/// [`multi_record`]: Features::multi_record
/// [`extended_memory`]: Features::extended_memory
/// [`lockable`]: Features::lockable
/// [`dynamic`]: Features::dynamic
#[derive(Copy, Clone, PartialEq, Eq, Hash, derive_new::new)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[repr(transparent)]
pub struct Features(pub u8);

impl Features {
    /// No features enabled.
    pub const EMPTY: Self = Self(0);

    /// Read-only NDEF data.
    pub const READ_ONLY: u8 = 1 << 0;

    /// Supports multi-record NDEF messages.
    pub const MULTI_RECORD: u8 = 1 << 1;

    /// Tag supports extended memory (> 255 bytes).
    pub const EXTENDED_MEMORY: u8 = 1 << 2;

    /// Tag supports lock-byte areas.
    pub const LOCKABLE: u8 = 1 << 3;

    /// Tag supports dynamic features.
    pub const DYNAMIC: u8 = 1 << 4;

    /// Enable a feature bit.
    #[inline(always)]
    pub fn enable(&mut self, bit: u8) {
        self.0 |= bit;
    }

    /// Disable a feature bit.
    #[inline(always)]
    pub fn disable(&mut self, bit: u8) {
        self.0 &= !bit;
    }

    /// Returns `true` if no feature bits are set.
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if the feature bit is set.
    #[inline(always)]
    pub const fn has(&self, bit: u8) -> bool {
        (self.0 & bit) != 0
    }

    #[inline(always)]
    pub const fn read_only(&self) -> bool {
        self.has(Self::READ_ONLY)
    }
    #[inline(always)]
    pub const fn multi_record(&self) -> bool {
        self.has(Self::MULTI_RECORD)
    }
    #[inline(always)]
    pub const fn extended_memory(&self) -> bool {
        self.has(Self::EXTENDED_MEMORY)
    }
    #[inline(always)]
    pub const fn lockable(&self) -> bool {
        self.has(Self::LOCKABLE)
    }
    #[inline(always)]
    pub const fn dynamic(&self) -> bool {
        self.has(Self::DYNAMIC)
    }

    #[inline(always)]
    pub const fn with(self, bit: u8) -> Self {
        Self(self.0 | bit)
    }

    #[inline(always)]
    pub const fn without(self, bit: u8) -> Self {
        Self(self.0 & !bit)
    }
}

impl Default for Features {
    /// Returns an empty [`Features`] value.
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl core::fmt::Debug for Features {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut first = true;

        macro_rules! show {
            ($flag:ident) => {
                if self.$flag() {
                    if !first {
                        write!(f, " | ")?;
                    }
                    write!(f, stringify!($flag))?;
                    first = false;
                }
            };
        }

        write!(f, "Features(")?;
        show!(read_only);
        show!(multi_record);
        show!(extended_memory);
        show!(lockable);
        show!(dynamic);

        if first {
            write!(f, "empty")?;
        }

        write!(f, ")")
    }
}

impl core::ops::BitOr for Features {
    type Output = Features;

    #[inline]
    fn bitor(self, rhs: Self) -> Self::Output {
        Features(self.0 | rhs.0)
    }
}

impl core::ops::BitAnd for Features {
    type Output = Features;

    #[inline]
    fn bitand(self, rhs: Self) -> Self::Output {
        Features(self.0 & rhs.0)
    }
}

impl core::ops::BitXor for Features {
    type Output = Features;

    #[inline]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Features(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for Features {
    type Output = Features;

    #[inline]
    fn not(self) -> Self::Output {
        Features(!self.0)
    }
}

impl From<u8> for Features {
    #[inline(always)]
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<Features> for u8 {
    #[inline(always)]
    fn from(f: Features) -> u8 {
        f.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Features {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Features {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Features(u8::deserialize(deserializer)?))
    }
}
