use super::Compat;
use futures01::Future as Future01;

impl<Fut: Future01> Future01CompatExt for Fut {}

/// Extension trait for futures 0.1 [`Future`](futures01::future::Future)
pub trait Future01CompatExt: Future01 {
    /// Converts a futures 0.1
    /// [`Future<Item = T, Error = E>`](futures01::future::Future)
    /// into a futures 0.3
    /// [`Future<Output = Result<T, E>>`](futures_core::future::Future).
    fn compat(self) -> Compat<Self, ()> where Self: Sized {
        Compat::new(self, None)
    }
}
