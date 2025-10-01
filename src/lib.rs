#![no_std]

use core::fmt::{self, Debug, Formatter};

/// A formatting function you can reference from `educe(Debug(method = "..."))`.
///
/// Example:
///   #[educe(Debug(method = "compact_debug::fmt_compact"))]
///   inner: Inner,
pub fn fmt_compact<T: Debug>(t: &T, f: &mut Formatter<'_>) -> fmt::Result {
    // Always compact:
    write!(f, "{:?}", t)
}

/// Newtype wrapper that forces compact `Debug` no matter how it's printed.
pub struct Compact<T>(pub T);

impl<T: Debug> Debug for Compact<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.0)
    }
}

/// Borrowing adapter to avoid moving/allocating: `AsCompact(&value)`.
pub struct AsCompact<'a, T: ?Sized>(pub &'a T);

impl<'a, T: Debug + ?Sized> Debug for AsCompact<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

/// Convenience trait to attach `.compact()` to any `Debug` value.
pub trait DebugExt: Debug + Sized {
    fn compact(self) -> Compact<Self> {
        Compact(self)
    }
    fn as_compact(&self) -> AsCompact<'_, Self> {
        AsCompact(self)
    }
}

impl<T: Debug> DebugExt for T {}
