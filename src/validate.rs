use crate::{error::ImplementationError, trace::MaybeDebug};

pub trait Validate: Sized + MaybeDebug {
    /// Error type returned by validation functions.
    type Error: ImplementationError;

    #[inline(always)]
    fn error(&self) -> Option<Self::Error> {
        None
    }

    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.error().is_none()
    }
}
