use arithmetic_mode::{panicking, wrapping, saturating, checked};

#[test]
fn test_panicking_u8_add_u8() {
    assert_eq!(15, panicking! { 5_u8 + 10_u8 });
}

#[test]
#[should_panic]
fn test_panicking_u8_add_u8_overflow() {
    panicking! { 5_u8 + 255_u8 };
}

#[test]
#[should_panic]
fn test_panicking_u8_add_u8_2() {
    panicking! { 200_u8 + 30_u8 + 30_u8 };
}

#[test]
fn test_panicking_operator_precedence() {
    assert_eq!(11, panicking! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
}

#[test]
fn test_wrapping_add() {
    assert_eq!(15, wrapping! { 5_u8 + 10_u8 });
}

#[test]
fn test_wrapping_add_overflow() {
    assert_eq!(4, wrapping! { 5_u8 + 255_u8 });
    assert_eq!(4, wrapping! { 200_u8 + 30_u8 + 30_u8 });
    assert_eq!(u32::MAX, wrapping! { 0_u32 - 1 });
}

#[test]
fn test_wrapping_operator_precedence() {
    assert_eq!(11, wrapping! { 1_u8 + 2_u8 * 3_u8 + 4_u8 });
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

#[test]
fn test_unchanged_expressions() {
    assert_eq!(42, panicking! { 42 });
    assert_eq!(-42, panicking! { -42 });
    assert_eq!(3, panicking! { 1 | 2 });
    assert_eq!(3, panicking! { 1 ^ 2 });
    assert_eq!(2, panicking! { -10 & 2 });

    assert_eq!(42, wrapping! { 42 });
    assert_eq!(-42, wrapping! { -42 });
    assert_eq!(3, wrapping! { 1 | 2 });
    assert_eq!(3, wrapping! { 1 ^ 2 });
    assert_eq!(2, wrapping! { -10 & 2 });

    assert_eq!(42, saturating! { 42 });
    assert_eq!(-42, saturating! { -42 });
    assert_eq!(3, saturating! { 1 | 2 });
    assert_eq!(3, saturating! { 1 ^ 2 });
    assert_eq!(2, saturating! { -10 & 2 });

    assert_eq!(Some(42), checked! { 42 });
    assert_eq!(Some(-42), checked! { -42 });
    assert_eq!(Some(3), checked! { 1 | 2 });
    assert_eq!(Some(3), checked! { 1 ^ 2 });
    assert_eq!(Some(2), checked! { -10 & 2 });
}
