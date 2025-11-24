/// A value paired with the number of bytes consumed while decoding it.
///
/// It represents:
/// - the number of bytes consumed (`usize`)
/// - the parsed value `T`
///
/// The `usize` is the offset immediately *after* the parsed value
/// (i.e., total bytes consumed up to and including this item).
pub type Consumed<T> = (usize, T);

/// A result that returns a parsed value together with the number of
/// bytes consumed while parsing it.
///
/// On success, this returns [`Consumed`]:
/// - the number of bytes consumed (`usize`)
/// - the parsed value `T`
///
/// On error, an error of type `E` is returned.
pub type ConsumedResult<E> = Result<usize, E>;

/// A result that returns only the number of bytes consumed.
pub type OffsetResult<T, E> = Result<Consumed<T>, E>;

/// Convenience alias for the crate’s standard [`Result`][crate::Result] type
/// returning a parsed value together with the number of bytes consumed.
///
/// # See also
/// - [`ConsumedResult`]
pub type CrateConsumed<T> = crate::Result<Consumed<T>>;
