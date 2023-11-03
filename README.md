# Macros for changing arithmetic modes (wrapping / saturating / checked etc)

Provides `checked!`, `panicking!`, `wrapping!` and `saturating!` macros to change how arithmetic overflows are handled.

> This crate currently not published on crates.io

## Example

```rust
use arithmetic_mode::wrapping;

wrapping! { 1_i32 + 2_i32 - 3_i32 };
```

## Supported operations:
* Add `+`
* Sub `-`
* Mul `*`
* Div `/`
* Shl `<<` (except `saturating`, due to https://github.com/rust-lang/libs-team/issues/230)
* Shr `>>` (except `saturating`, due to https://github.com/rust-lang/libs-team/issues/230)
## Known issues
* For most operations, constraining the numeric literals are required (e.g.
  `2_i32` instead of `2`), due to
  <https://github.com/rust-lang/rust/issues/24124>.
