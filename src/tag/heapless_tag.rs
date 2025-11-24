use crate::tag::{message::DecodeMessage, Block, CapabilityContainer, RawTag};

/// A heapless TLV tag containing decoded blocks and a Capability Container.
///
/// This is the TLV analogue of [`HeaplessMessage`], storing:
/// - a parsed Capability Container (`cc`)
/// - a heapless list of TLV blocks (`blocks`)
///
/// # Lifetimes
/// - `'t` -- lifetime of the raw TLV buffer
///
///
/// # Type Parameters
/// - `B` -- TLV block decoder implementing [`DecodeMessage`]
/// - `L` -- maximum number of TLV blocks
///
/// Useful for NFC Type 2 / 4 tag processing in embedded systems.
#[derive(Debug, Clone)]
pub struct HeaplessTag<'t, B: DecodeMessage<'t>, const L: usize> {
    pub cc: CapabilityContainer,
    pub blocks: heapless::Vec<Block<'t, B>, L>,
}

impl<'t, B: DecodeMessage<'t>, const L: usize> HeaplessTag<'t, B, L> {
    #[inline]
    pub fn new(cc: CapabilityContainer, blocks: heapless::Vec<Block<'t, B>, L>) -> Self {
        Self { cc, blocks }
    }

    pub fn decode(bytes: &'t [u8]) -> crate::Result<Self> {
        Self::try_from(RawTag::decode(bytes)?)
    }
}

impl<'t, B: DecodeMessage<'t>, const L: usize> TryFrom<RawTag<'t>> for HeaplessTag<'t, B, L> {
    type Error = crate::Error;

    fn try_from(t: RawTag<'t>) -> Result<Self, Self::Error> {
        let mut blocks = heapless::Vec::new();

        for block in t.blocks::<B>() {
            blocks.push(block?).map_err(|_| crate::Error::VecOutOfSpace(L))?;
        }

        Ok(Self { cc: t.cc, blocks })
    }
}

#[cfg(feature = "serde")]
impl<'t, B, const L: usize> serde::Serialize for HeaplessTag<'t, B, L>
where
    B: DecodeMessage<'t>,
    Block<'t, B>: serde::Serialize,
{
    /// Serialize the tag into a struct.
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("Tag", 2)?;
        st.serialize_field("cc", &self.cc)?;
        st.serialize_field("blocks", &self.blocks)?;
        st.end()
    }
}

#[cfg(feature = "serde")]
impl<'t, B, const L: usize> serde::Deserialize<'t> for HeaplessTag<'t, B, L>
where
    B: DecodeMessage<'t>,
    Block<'t, B>: serde::Deserialize<'t>,
{
    /// Deserialize the tag from a struct.
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'t>,
    {
        #[derive(serde::Deserialize)]
        struct HeaplessTagRaw<'t, CC, BL> {
            cc: CC,
            blocks: BL,

            #[serde(skip)]
            _marker: core::marker::PhantomData<&'t ()>,
        }

        let HeaplessTagRaw { cc, blocks, .. } =
            HeaplessTagRaw::<_, heapless::Vec<_, L>>::deserialize(d)?;
        Ok(Self { cc, blocks })
    }
}
