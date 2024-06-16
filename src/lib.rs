pub mod endpoint;

use std::{cmp::Ordering, ops::Mul};

use endpoint::Endpoint;

#[cfg(feature = "serde")]
mod _interval {
    use serde::{Deserialize, Serialize};

    use crate::endpoint::Endpoint;

    #[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct Interval<T> {
        pub left: Endpoint<T>,
        pub right: Endpoint<T>,
    }
}

#[cfg(not(feature = "serde"))]
mod _interval {
    use crate::endpoint::Endpoint;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Interval<T> {
        pub left: Endpoint<T>,
        pub right: Endpoint<T>,
    }
}

pub use _interval::Interval;

impl<T> Interval<T> {
    pub const fn new(left: Endpoint<T>, right: Endpoint<T>) -> Self {
        Self { left, right }
    }
    /// Empty set.
    pub const EMPTY: Self = Self::new(Endpoint::PosInf, Endpoint::NegInf);
    /// Universe set.
    pub const ALL: Self = Self::new(Endpoint::NegInf, Endpoint::PosInf);
    /// Interval [`x`, +inf).
    pub const fn ge(x: T) -> Self {
        Self::new(Endpoint::Closed(x), Endpoint::PosInf)
    }
    /// Interval (-inf, `x`].
    pub const fn le(x: T) -> Self {
        Self::new(Endpoint::NegInf, Endpoint::Closed(x))
    }
    /// Interval (`x`, +inf).
    pub const fn gt(x: T) -> Self {
        Self::new(Endpoint::LOpen(x), Endpoint::PosInf)
    }
    /// Interval (-inf, `x`).
    pub const fn lt(x: T) -> Self {
        Self::new(Endpoint::NegInf, Endpoint::ROpen(x))
    }
    /// Open interval (`left`, `right`).
    pub const fn open(left: T, right: T) -> Self {
        Self::new(Endpoint::LOpen(left), Endpoint::ROpen(right))
    }
    /// Closed interval \[`left`, `right`\].
    pub const fn closed(left: T, right: T) -> Self {
        Self::new(Endpoint::Closed(left), Endpoint::Closed(right))
    }
    /// Left-closed-right-open interval [`left`, `right`).
    pub const fn lcro(left: T, right: T) -> Self {
        Self::new(Endpoint::Closed(left), Endpoint::ROpen(right))
    }
    /// Left-open-right-closed interval (`left`, `right`].
    pub const fn lorc(left: T, right: T) -> Self {
        Self::new(Endpoint::LOpen(left), Endpoint::Closed(right))
    }
}

impl<T: Copy> Interval<T> {
    /// Interval containing only one element, acts as \[`x`, `x`\].
    pub const fn only(x: T) -> Self {
        Self::closed(x, x)
    }
}

impl<T> Interval<T>
where
    Endpoint<T>: PartialOrd,
{
    pub fn is_empty(&self) -> bool {
        self.left > self.right
    }
}

impl<T> Interval<T>
where
    Self: PartialEq,
{
    pub fn is_all(&self) -> bool {
        *self == Self::ALL
    }
}

impl<T> Interval<T>
where
    Endpoint<T>: PartialOrd,
{
    pub fn contain(&self, value: T) -> bool {
        let v = Endpoint::Closed(value);
        v >= self.left && v <= self.right
    }
}

#[rustfmt::skip]
impl<T: Eq> PartialOrd for Interval<T>
where
    Endpoint<T>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_empty() && other.is_empty() { return Some(Ordering::Equal) }
        if self.left == other.left && self.right == other.right { return Some(Ordering::Equal) }
        if self.left >= other.left && self.right <= other.right { return Some(Ordering::Less) }
        if self.left <= other.left && self.right >= other.right { return Some(Ordering::Greater) }
        None
    }
}

impl<T> Mul for Interval<T>
where
    Endpoint<T>: Ord,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.left.max(rhs.left), self.right.min(rhs.right))
    }
}

#[cfg(test)]
mod test {
    use crate::Interval;

    #[test]
    fn interval_ops() {
        assert!(Interval::closed(1, 2) > Interval::open(1, 2));
        assert!(Interval::closed(1, 3) * Interval::open(2, 4) == Interval::lorc(2, 3));
        assert!(Interval::EMPTY >= Interval::open(1, 1));
    }
}
