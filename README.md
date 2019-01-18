# sensible-dbg

If you read [the discussion on `std`'s new `dbg!` macro](https://github.com/rust-lang/rfcs/pull/2361),
you'll find that there are good reasons why it shouldn't affect release builds.
As a silent protest against the decision, I made my own version.


# Usage

```rust
use sensible_dbg::dbg;

fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {
        dbg!(1)
    } else {
        dbg!(n * factorial(n - 1))
    }
}
```
