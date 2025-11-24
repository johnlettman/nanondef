use core::{
    cmp::min,
    ops::{Add, Bound, Index, IndexMut, RangeBounds},
};

/// A half-open range of `usize` values represented as `(start, end)`.
///
/// `Range` stores a pair of indices where the first element is inclusive and
/// the second element is exclusive, matching Rust’s typical `start..end`
/// convention. No validation is performed, so callers are responsible for
/// ensuring `start <= end`.
///
/// The type derives `Ord` and `Eq`, and also implements `PartialOrd` based on
/// range length rather than start/end position. This makes `Range` comparable
/// by size, not by lexical order of its bounds.
///
/// # Examples
/// ```
/// use nanondef::Range;
/// let r = Range::new(2, 5);
/// assert_eq!(r.len(), 3);
/// assert_eq!(format!("{}", r), "2..5");
/// ```
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    /// Creates a new `Range` with the given inclusive `from` bound and
    /// exclusive `to` bound.
    ///
    /// The caller must ensure that `from <= to`. No bounds checking or
    /// validation is performed inside this constructor.
    ///
    /// # Parameters
    /// - `from`: inclusive start index
    /// - `to`: exclusive end index
    ///
    /// # Returns
    /// A `Range` representing `from..to`.
    #[inline(always)]
    pub const fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    #[inline(always)]
    pub const fn to(end: usize) -> Self {
        Self { start: 0, end }
    }

    /// Returns the length of the range (`end - start`).
    ///
    /// This computes the number of elements covered by the half-open interval
    /// `start..end`. No validation is performed; if the range is invalid
    /// (e.g. `start > end`), the result will wrap, according to `usize`
    /// subtraction rules.
    ///
    /// # Example
    /// ```
    /// use nanondef::Range;
    /// let r = Range::new(10, 14);
    /// assert_eq!(r.len(), 4);
    /// ```
    #[inline(always)]
    pub const fn len(&self) -> usize {
        debug_assert!(self.end <= self.start);
        self.end - self.start
    }

    #[inline(always)]
    pub const fn as_core(&self) -> core::ops::Range<usize> {
        core::ops::Range { start: self.start, end: self.end }
    }

    #[inline(always)]
    pub const fn from_core(r: core::ops::Range<usize>) -> Self {
        Self { start: r.start, end: r.end }
    }
}

impl core::fmt::Display for Range {
    /// Formats the range as `"start..end"`.
    ///
    /// # Example
    /// ```
    /// use nanondef::Range;
    /// let r = Range::new(3, 9);
    /// assert_eq!(r.to_string(), "3..9");
    /// ```
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl Add<Range> for Range {
    type Output = Range;

    fn add(self, rhs: Range) -> Self::Output {
        Self { start: min(self.start, rhs.start), end: self.len() + rhs.len() }
    }
}

impl Add<usize> for Range {
    type Output = Range;

    fn add(self, rhs: usize) -> Self::Output {
        Self { start: self.start, end: self.end + rhs }
    }
}

impl Index<Range> for [u8] {
    type Output = [u8];

    fn index(&self, range: Range) -> &[u8] {
        &self[range.start..range.end]
    }
}

impl IndexMut<Range> for [u8] {
    fn index_mut(&mut self, range: Range) -> &mut Self::Output {
        &mut self[range.start..range.end]
    }
}

impl RangeBounds<usize> for Range {
    #[inline]
    fn start_bound(&self) -> Bound<&usize> {
        Bound::Included(&self.start)
    }

    #[inline]
    fn end_bound(&self) -> Bound<&usize> {
        Bound::Excluded(&self.end)
    }
}

impl PartialOrd for Range {
    /// Compares two ranges by their length.
    ///
    /// This returns the result of comparing `self.len()` and `other.len()`.
    /// Ordering is therefore *not* based on the start or end bounds.
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.len().partial_cmp(&other.len())
    }
}

impl From<Range> for (usize, usize) {
    #[inline(always)]
    fn from(r: Range) -> Self {
        (r.start, r.end)
    }
}

impl From<(usize, usize)> for Range {
    #[inline(always)]
    fn from(v: (usize, usize)) -> Self {
        Self { start: v.0, end: v.1 }
    }
}

impl From<core::ops::Range<usize>> for Range {
    #[inline(always)]
    fn from(r: core::ops::Range<usize>) -> Self {
        Self::from_core(r)
    }
}

impl From<Range> for core::ops::Range<usize> {
    #[inline(always)]
    fn from(r: Range) -> Self {
        r.as_core()
    }
}
