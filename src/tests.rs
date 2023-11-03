use crate::{checked_impl, panicking_impl, saturating_impl, wrapping_impl};
use proc_macro_utils::assert_expansion;
use quote::quote;

#[test]
fn test_add() {
    assert_expansion!(panicking_impl! { 42 + 55 }.unwrap(), {
        (42).checked_add(55).unwrap()
    });
    assert_expansion!(wrapping_impl! { 42 + 55 }.unwrap(), {
        (42).wrapping_add(55)
    });
    assert_expansion!(saturating_impl! { 42 + 55 }.unwrap(), {
        (42).saturating_add(55)
    });
    assert_expansion!(checked_impl! { 42 + 55 }.unwrap(), {
        (Some(42)).zip(Some(55)).and_then(|(l, r)| l.checked_add(r))
    });
}

#[test]
fn test_sub() {
    assert_expansion!(panicking_impl! { 42 - 55 }.unwrap(), {
        (42).checked_sub(55).unwrap()
    });
    assert_expansion!(wrapping_impl! { 42 - 55 }.unwrap(), {
        (42).wrapping_sub(55)
    });
    assert_expansion!(saturating_impl! { 42 - 55 }.unwrap(), {
        (42).saturating_sub(55)
    });
    assert_expansion!(checked_impl! { 42 - 55 }.unwrap(), {
        (Some(42)).zip(Some(55)).and_then(|(l, r)| l.checked_sub(r))
    });
}

#[test]
fn test_mul() {
    assert_expansion!(panicking_impl! { 42 * 55 }.unwrap(), {
        (42).checked_mul(55).unwrap()
    });
    assert_expansion!(wrapping_impl! { 42 * 55 }.unwrap(), {
        (42).wrapping_mul(55)
    });
    assert_expansion!(saturating_impl! { 42 * 55 }.unwrap(), {
        (42).saturating_mul(55)
    });
    assert_expansion!(checked_impl! { 42 * 55 }.unwrap(), {
        (Some(42)).zip(Some(55)).and_then(|(l, r)| l.checked_mul(r))
    });
}

#[test]
fn test_compound() {
    assert_expansion!(panicking_impl! { 42 - 55 + 121 }.unwrap(), {
        ((42).checked_sub(55).unwrap()).checked_add(121).unwrap()
    });
    assert_expansion!(wrapping_impl! { 42 - 55 + 121 }.unwrap(), {
        ((42).wrapping_sub(55)).wrapping_add(121)
    });
    assert_expansion!(saturating_impl! { 42 - 55 + 121 }.unwrap(), {
        ((42).saturating_sub(55)).saturating_add(121)
    });
    assert_expansion!(checked_impl! { 42 - 55 + 121 }.unwrap(), {
        ((Some(42)).zip(Some(55)).and_then(|(l, r)| l.checked_sub(r)))
            .zip(Some(121))
            .and_then(|(l, r)| l.checked_add(r))
    });
}

#[test]
fn test_unchanged_expressions() {
    assert_expansion!(panicking_impl! { 42 }.unwrap(), { 42 });
    assert_expansion!(panicking_impl! { -42 }.unwrap(), { -42 });
    assert_expansion!(panicking_impl! { 1 || 2 }.unwrap(), { 1 || 2 });
    assert_expansion!(panicking_impl! { 1 | 2 }.unwrap(), { 1 | 2 });
    assert_expansion!(panicking_impl! { 1 ^ 2 }.unwrap(), { 1 ^ 2 });
    assert_expansion!(panicking_impl! { -1 & 2 }.unwrap(), { -1 & 2 });

    assert_expansion!(wrapping_impl! { 42 }.unwrap(), { 42 });
    assert_expansion!(wrapping_impl! { -42 }.unwrap(), { -42 });
    assert_expansion!(wrapping_impl! { 1 || 2 }.unwrap(), { 1 || 2 });
    assert_expansion!(wrapping_impl! { 1 | 2 }.unwrap(), { 1 | 2 });
    assert_expansion!(wrapping_impl! { 1 ^ 2 }.unwrap(), { 1 ^ 2 });
    assert_expansion!(wrapping_impl! { -1 & 2 }.unwrap(), { -1 & 2 });

    assert_expansion!(saturating_impl! { 42 }.unwrap(), { 42 });
    assert_expansion!(saturating_impl! { -42 }.unwrap(), { -42 });
    assert_expansion!(saturating_impl! { 1 || 2 }.unwrap(), { 1 || 2 });
    assert_expansion!(saturating_impl! { 1 | 2 }.unwrap(), { 1 | 2 });
    assert_expansion!(saturating_impl! { 1 ^ 2 }.unwrap(), { 1 ^ 2 });
    assert_expansion!(saturating_impl! { -1 & 2 }.unwrap(), { -1 & 2 });

    assert_expansion!(checked_impl! { 42 }.unwrap(), { Some(42) });
    assert_expansion!(checked_impl! { -42 }.unwrap(), { (Some(42)).map(|v| -v) });
    assert_expansion!(checked_impl! { 1 || 2 }.unwrap(), {
        (Some(1)).zip(Some(2)).map(|(l, r)| l || r)
    });
    assert_expansion!(checked_impl! { 1 | 2 }.unwrap(), {
        (Some(1)).zip(Some(2)).map(|(l, r)| l | r)
    });
    assert_expansion!(checked_impl! { 1 ^ 2 }.unwrap(), {
        (Some(1)).zip(Some(2)).map(|(l, r)| l ^ r)
    });
    assert_expansion!(checked_impl! { -1 & 2 }.unwrap(), {
        ((Some(1)).map(|v| -v)).zip(Some(2)).map(|(l, r)| l & r)
    });
}

#[test]
fn test_bitshift() {
    assert_expansion!(panicking_impl! { 1 << 2 >> 3 }.unwrap(), {
        ((1).checked_shl(2).unwrap()).checked_shr(3).unwrap()
    });
    assert_expansion!(wrapping_impl! { 1 << 2 >> 3 }.unwrap(), {
        ((1).wrapping_shl(2)).wrapping_shr(3)
    });
    assert_expansion!(checked_impl! { 1 << 2 >> 3 }.unwrap(), {
        ((Some(1)).zip(Some(2)).and_then(|(l, r)| l.checked_shl(r)))
            .zip(Some(3))
            .and_then(|(l, r)| l.checked_shr(r))
    });
}

#[test]
fn test_bitshift_no_saturating() {
    saturating_impl(quote! { 1 << 2 >> 3 }).unwrap_err();
}
