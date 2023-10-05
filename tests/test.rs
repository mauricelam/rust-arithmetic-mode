#![no_std]

use arithmetic_mode::{checked, panicking, saturating, wrapping};
use paste::paste;

#[test]
fn test_panicking() {
    assert_eq!(15, panicking! { 5_u8 + 10_u8 });
    assert_eq!(11, panicking! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
}

#[test]
#[should_panic]
fn test_u8_add_u8() {
    panicking! { 5_u8+255_u8 };
}

#[test]
#[should_panic]
fn test_u8_add_u8_2() {
    panicking! { 200_u8+30_u8+30_u8 };
}

#[test]
fn test_wrapping() {
    assert_eq!(15, wrapping! { 5_u8 + 10_u8 });
    assert_eq!(11, wrapping! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
    assert_eq!(1, wrapping! { -1_i8 + 2 });
    assert_eq!(-128, wrapping! { -1i8 << 7 });
    assert_eq!(-1, wrapping! { -1i8 << 128 });
    assert_eq!(-1, wrapping! { -128_i8 >> 7 });
    assert_eq!(-128, wrapping! { -128_i16 >> 64 });
}

#[test]
fn test_wrapping_add_overflow() {
    assert_eq!(4, wrapping! { 5_u8 + 255_u8 });
    assert_eq!(4, wrapping! { 200_u8 + 30_u8 + 30_u8 });
    assert_eq!(u32::MAX, wrapping! { 0_u32 - 1 });
}

#[test]
fn test_saturating_add() {
    assert_eq!(15, saturating! { 5_u8 + 10_u8 });
}

#[test]
fn test_saturating_add_overflow() {
    assert_eq!(255, saturating! { 5_u8 + 255_u8 });
    assert_eq!(255, saturating! { 200_u8 + 30_u8 + 30_u8 });
    assert_eq!(0, saturating! { 0_u32 - 1 });
}

#[test]
fn test_saturating_operator_precedence() {
    assert_eq!(11, saturating! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
}

#[test]
fn test_checked_add() {
    assert_eq!(Some(15), checked! { 5_u8 + 10_u8 });
}

#[test]
fn test_checked_add_overflow() {
    assert_eq!(None, checked! { 5_u8 + 255_u8 });
    assert_eq!(None, checked! { 200_u8 + 30_u8 + 30_u8 });
    assert_eq!(None, checked! { 0_u32 - 1 });
}

#[test]
fn test_checked_operator_precedence() {
    assert_eq!(Some(11), checked! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
}

macro_rules! test_unchanging {
    ($ident:ident, $expr:expr) => {
        paste! {
            #[test]
            fn [<test_unchanging_panicking_ $ident>]() {
                assert_eq!(($expr), (panicking! { $expr }));
            }

            #[test]
            fn [<test_unchanging_wrapped_ $ident>]() {
                assert_eq!($expr, wrapping!{ $expr });
            }

            #[test]
            fn [<test_unchanging_saturating_ $ident>]() {
                assert_eq!($expr, saturating!{ $expr });
            }

            #[test]
            fn [<test_unchanging_checked_ $ident>]() {
                assert_eq!(Some($expr), checked!{ $expr });
            }
        }
    };
}

test_unchanging!(constant, 42);
test_unchanging!(negate, -42);
test_unchanging!(or, 1 | 2);
test_unchanging!(and, -10 & 2);
test_unchanging!(xor, 1 ^ 2);
test_unchanging!(complex, (1 ^ 2) | (3_i32 + (-4_i32 * 5_i32)));
