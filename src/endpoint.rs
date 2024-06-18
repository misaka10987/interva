#[cfg(feature = "serde")]
mod _endpoint {
    use serde::{Deserialize, Serialize};

    /// Defines an endpoint of an interval.
    ///
    /// `interva` performs a trick here that it handles `LOpen(x)` as `x`+*Infinitesimal* and `ROpen(x)` as `x`-*Infinitesimal*.
    ///
    /// `Endpoint<T>` implements `PartialOrd` as long as `T` implements it, making the following guarantees:
    /// - for `x:T`, `NegInf` < `ROpen(x)` < `Closed(x)` < `LOpen(x)` < `PosInf`
    /// - for `x:T`, `y:T` and `x<y`, `Endpoint<T>` with value `x` is always less than `Endpoint<T>` with value `y`, regardless of `LOpen`, `ROpen` or `Closed`
    /// - `PosInf == Posinf`, `NegInf == NegInf`
    ///
    /// It can be proved from above that `Endpoint<T>` is `Ord` if `T` is.
    #[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Endpoint<T> {
        /// []
        Closed(T),
        /// (,
        LOpen(T),
        /// ,)
        ROpen(T),
        PosInf,
        NegInf,
    }
}

#[cfg(not(feature = "serde"))]
mod _endpoint {
    /// Defines an endpoint of an interval.
    ///
    /// `interva` performs a trick here that it handles `LOpen(x)` as `x`+*Infinitesimal* and `ROpen(x)` as `x`-*Infinitesimal*.
    ///
    /// `Endpoint<T>` implements `PartialOrd` as long as `T` implements it, making the following guarantees:
    /// - for `x:T`, `NegInf` < `ROpen(x)` < `Closed(x)` < `LOpen(x)` < `PosInf`
    /// - for `x:T`, `y:T` and `x<y`, `Endpoint<T>` with value `x` is always less than `Endpoint<T>` with value `y`, regardless of `LOpen`, `ROpen` or `Closed`
    /// - `PosInf == Posinf`, `NegInf == NegInf`
    ///
    /// It can be proved from above that `Endpoint<T>` is `Ord` if `T` is.
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Endpoint<T> {
        /// []
        Closed(T),
        /// (,
        LOpen(T),
        /// ,)
        ROpen(T),
        PosInf,
        NegInf,
    }
}

use std::cmp::Ordering;

pub use _endpoint::Endpoint;

fn cmp_or<T: PartialOrd>(x: T, y: T, eq_fallback: Ordering) -> Option<Ordering> {
    if let Some(ord) = x.partial_cmp(&y) {
        if ord == Ordering::Equal {
            Some(eq_fallback)
        } else {
            Some(ord)
        }
    } else {
        None
    }
}

const GT: Ordering = Ordering::Greater;
const LT: Ordering = Ordering::Less;

impl<T> PartialOrd for Endpoint<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Endpoint::Closed(x), Endpoint::Closed(y)) => x.partial_cmp(y),
            (Endpoint::LOpen(x), Endpoint::LOpen(y)) => x.partial_cmp(y),
            (Endpoint::ROpen(x), Endpoint::ROpen(y)) => x.partial_cmp(y),

            (Endpoint::Closed(x), Endpoint::LOpen(y)) => cmp_or(x, y, LT),
            (Endpoint::Closed(x), Endpoint::ROpen(y)) => cmp_or(x, y, GT),
            (Endpoint::LOpen(x), Endpoint::Closed(y)) => cmp_or(x, y, GT),
            (Endpoint::LOpen(x), Endpoint::ROpen(y)) => cmp_or(x, y, GT),
            (Endpoint::ROpen(x), Endpoint::Closed(y)) => cmp_or(x, y, LT),
            (Endpoint::ROpen(x), Endpoint::LOpen(y)) => cmp_or(x, y, LT),

            (Endpoint::PosInf, Endpoint::PosInf) => Some(Ordering::Equal),
            (Endpoint::NegInf, Endpoint::NegInf) => Some(Ordering::Equal),
            (Endpoint::PosInf, _) => Some(Ordering::Greater),
            (Endpoint::NegInf, _) => Some(Ordering::Less),
            (_, Endpoint::PosInf) => Some(Ordering::Less),
            (_, Endpoint::NegInf) => Some(Ordering::Greater),
        }
    }
}

impl<T> Ord for Endpoint<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
