# interva
A feature-rich crate for handling intervals.
## Example
```rust
use interva::Interval;

// proper-subset
assert!(Interval::closed(1, 2) > Interval::open(1, 2));
// subset
assert!(Interval::<i32>::EMPTY <= Interval::EMPTY); // `i32`'s here just because type inference failed
// intersection
assert!(Interval::closed(1, 3) * Interval::open(2, 4) == Interval::lorc(2, 3)); // `lorc` for "left-open-right-closed"
// use `/` to judge whether contains certain element
assert!(Interval::closed(1.5, 1.7) / 1.7);
assert!(!(Interval::open(1.5, 1.7) / 1.7));
```
## Features
`serde` for optional serialization support.