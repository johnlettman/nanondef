
use core::cmp::Ordering as CoreOrdering;

#[repr(i8)]
pub enum Ordering {
    Less = -1,
    Equal = 0,
    Greater = 1
}

impl From<CoreOrdering> for Ordering {
    fn from(o: CoreOrdering) -> Self {
        match o {
            CoreOrdering::Less => Ordering::Less,
            CoreOrdering::Equal => Ordering::Equal,
            CoreOrdering::Greater => Ordering::Greater,
        }
    }
}
