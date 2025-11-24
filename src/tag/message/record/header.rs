use crate::tag::message::record::Tnf;

/// The parsed header fields of an NDEF record.
///
/// Contains:
/// - The optional **ID** field (`id`)
/// - The **TNF** (Type Name Format)
/// - The **Type** string (`ty`)
///
/// All fields borrow from the underlying NDEF buffer via lifetime `'r`.
///
/// This type is constructed during record parsing and reused by both typed and
/// untyped payload decoders.
#[derive(Debug, Clone, derive_new::new)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Header<'r> {
    /// Optional record ID field.
    pub id: Option<&'r [u8]>,

    /// The record Type Name Format (TNF) classification.
    pub tnf: Tnf,

    /// The record's Type field.
    pub ty: &'r str,
}
