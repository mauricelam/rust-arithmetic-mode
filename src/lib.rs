//! Macros to make different arithmetic modes (checked, wrapping, saturating,
//! panicking etc) more readable, overriding the
//! [default](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow).
//!
//! Instead of
//! ```rust
//! 1_i32.wrapping_add(2_i32).wrapping_sub(3_i32);
//! ```
//!
//! You can write
//! ```rust
//! # use arithmetic_mode::wrapping;
//!
//! wrapping! { 1_i32 + 2_i32 - 3_i32 };
//! ```
//!
//! ## Supported operations:
//! * Add `+`
//! * Sub `-`
//! * Mul `*`
//! * Div `/`
//! * Shl `<<` (except `saturating`, which is only supported with the feature
//!   `saturating_int_impl`, and requires nightly)
//! * Shr `>>` (except `saturating`, which is only supported with the feature
//!   `saturating_int_impl`, and requires nightly)
//!
//! ## Known issues
//! * For most operations, constraining the numeric literals are required (e.g.
//!   `2_i32` instead of `2`), due to
//!   <https://github.com/rust-lang/rust/issues/24124>.

#![warn(missing_docs)]

extern crate proc_macro;

use anyhow::bail;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprBinary, ExprGroup, ExprLit, ExprParen, ExprUnary};

/// Macro that changes the arithmetic operations (add, sub, mul, div, and shift)
/// inside it to be panicking operations. That is, if any of the operation
/// overflows, it will panic.
///
/// ```should_panic
/// use arithmetic_mode::panicking;
///
/// assert_eq!(0, panicking! { 255_u8 + 1_u8 });
/// ```
#[proc_macro]
pub fn panicking(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match panicking_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let error_message = format!("Error: {e}");
            quote! { compile_error!(#error_message) }.into()
        }
    }
}

fn panicking_impl(item: TokenStream) -> anyhow::Result<TokenStream> {
    fn recurse(expr: syn::Expr) -> anyhow::Result<TokenStream> {
        Ok(match expr {
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let new_left = recurse(*left)?;
                let new_right = recurse(*right)?;
                checked_op(op)?
                    .map(|func| quote! { (#new_left).#func(#new_right).unwrap() })
                    .unwrap_or_else(|| quote! { #new_left #op #new_right })
            }
            Expr::Cast(_) => quote! { #expr },
            // Recurse inside the following items
            Expr::Index(_) => quote! { #expr },
            Expr::Lit(_) => quote! { #expr },
            Expr::Paren(ExprParen { expr, .. }) => {
                let new_expr = recurse(*expr)?;
                quote! { ( #new_expr ) }
            }
            Expr::Path(_) => quote! { #expr },
            Expr::Reference(_) => quote! { #expr },
            Expr::Unary(_) => quote! { #expr },
            Expr::Group(ExprGroup { expr, .. }) => recurse(*expr)?,
            _ => bail!("Unexpected expression token {expr:?}"),
        })
    }

    recurse(syn::parse2(item)?)
}

/// Macro that changes the arithmetic operations (add, sub, mul, div, and shift)
/// inside it to be wrapping operations if any operations or intermediate step
/// overflows.
///
/// ```rust
/// use arithmetic_mode::wrapping;
///
/// assert_eq!(0, wrapping! { 255_u8 + 1_u8 });
/// ```
#[proc_macro]
pub fn wrapping(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match wrapping_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let error_message = format!("Error: {e}");
            quote! { compile_error!(#error_message) }.into()
        }
    }
}

fn wrapping_impl(item: TokenStream) -> anyhow::Result<TokenStream> {
    fn recurse(expr: syn::Expr) -> anyhow::Result<TokenStream> {
        Ok(match expr {
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let new_left = recurse(*left)?;
                let new_right = recurse(*right)?;
                wrapping_op(op)?
                    .map(|func| quote! { (#new_left).#func(#new_right) })
                    .unwrap_or_else(|| quote! { #new_left #op #new_right })
            }
            Expr::Cast(_) => quote! { #expr },
            // Recurse inside the following items
            Expr::Index(_) => quote! { #expr },
            Expr::Lit(_) => quote! { #expr },
            Expr::Paren(ExprParen { expr, .. }) => {
                let new_expr = recurse(*expr)?;
                quote! { ( #new_expr ) }
            }
            Expr::Path(_) => quote! { #expr },
            Expr::Reference(_) => quote! { #expr },
            Expr::Unary(_) => quote! { #expr },
            Expr::Group(ExprGroup { expr, .. }) => recurse(*expr)?,
            _ => bail!("Unexpected expression token {expr:?}"),
        })
    }

    recurse(syn::parse2(item)?)
}

/// Macro that changes the arithmetic operations (add, sub, mul, div, and shift)
/// inside it to be saturating operations if any operations or intermediate step
/// overflows.
///
/// ```rust
/// use arithmetic_mode::saturating;
///
/// assert_eq!(255, saturating! { 255_u8 + 1_u8 });
/// ```
#[proc_macro]
pub fn saturating(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match saturating_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let error_message = format!("Error: {e}");
            quote! { compile_error!(#error_message) }.into()
        }
    }
}

fn saturating_impl(item: TokenStream) -> anyhow::Result<TokenStream> {
    fn recurse(expr: syn::Expr) -> anyhow::Result<TokenStream> {
        Ok(match expr {
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let new_left = recurse(*left)?;
                let new_right = recurse(*right)?;
                match op {
                    syn::BinOp::Add(_) => quote! { (#new_left).saturating_add(#new_right) },
                    syn::BinOp::Sub(_) => quote! { (#new_left).saturating_sub(#new_right) },
                    syn::BinOp::Mul(_) => quote! { (#new_left).saturating_mul(#new_right) },
                    syn::BinOp::Div(_) => quote! { (#new_left).saturating_div(#new_right) },
                    syn::BinOp::Rem(_) => quote! { (#new_left).saturating_rem(#new_right) },
                    syn::BinOp::Shl(_) => {
                        #[cfg(feature = "saturating_int_impl")]
                        {
                            quote! { (::core::num::Saturating(#new_left) #op (#new_right)).0 }
                        }

                        #[cfg(not(feature = "saturating_int_impl"))]
                        {
                            bail!("Saturating bit shifts are not supported (https://github.com/rust-lang/libs-team/issues/230)")
                        }
                    }
                    syn::BinOp::Shr(_) => {
                        #[cfg(feature = "saturating_int_impl")]
                        {
                            quote! { (::core::num::Saturating(#new_left) #op (#new_right)).0 }
                        }

                        #[cfg(not(feature = "saturating_int_impl"))]
                        {
                            bail!("Saturating bit shifts are not supported (https://github.com/rust-lang/libs-team/issues/230)")
                        }
                    }
                    syn::BinOp::And(_)
                    | syn::BinOp::Or(_)
                    | syn::BinOp::BitXor(_)
                    | syn::BinOp::BitAnd(_)
                    | syn::BinOp::BitOr(_)
                    | syn::BinOp::Eq(_)
                    | syn::BinOp::Lt(_)
                    | syn::BinOp::Le(_)
                    | syn::BinOp::Ne(_)
                    | syn::BinOp::Ge(_)
                    | syn::BinOp::Gt(_) => quote! { #new_left #op #new_right },
                    syn::BinOp::AddAssign(_)
                    | syn::BinOp::SubAssign(_)
                    | syn::BinOp::MulAssign(_)
                    | syn::BinOp::DivAssign(_)
                    | syn::BinOp::RemAssign(_)
                    | syn::BinOp::BitXorAssign(_)
                    | syn::BinOp::BitAndAssign(_)
                    | syn::BinOp::BitOrAssign(_)
                    | syn::BinOp::ShlAssign(_)
                    | syn::BinOp::ShrAssign(_) => bail!("Unsupported operation"),
                    _ => bail!("Unknown op: {op:?}"),
                }
            }
            Expr::Cast(_) => quote! { #expr },
            // Recurse inside the following items
            Expr::Index(_) => quote! { #expr },
            Expr::Lit(_) => quote! { #expr },
            Expr::Paren(ExprParen { expr, .. }) => {
                let new_expr = recurse(*expr)?;
                quote! { ( #new_expr ) }
            }
            Expr::Path(_) => quote! { #expr },
            Expr::Reference(_) => quote! { #expr },
            Expr::Unary(_) => quote! { #expr },
            Expr::Group(ExprGroup { expr, .. }) => recurse(*expr)?,
            _ => bail!("Unexpected expression token {expr:?}"),
        })
    }

    recurse(syn::parse2(item)?)
}

/// Macro that changes the arithmetic operations (add, sub, mul, div, and shift)
/// inside it to be checked operations that returns an `Option`, which would be
/// `None` if the operation or any intermediate step overflows.
///
/// ```rust
/// use arithmetic_mode::checked;
///
/// assert_eq!(Some(254), checked! { 255_u8 - 1_u8 });
/// assert_eq!(None, checked! { 255_u8 + 1_u8 });
/// ```
#[proc_macro]
pub fn checked(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match checked_impl(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => {
            let error_message = format!("Error: {e}");
            quote! { compile_error!(#error_message) }.into()
        }
    }
}

fn checked_impl(item: TokenStream) -> anyhow::Result<TokenStream> {
    fn recurse(expr: syn::Expr) -> anyhow::Result<TokenStream> {
        Ok(match expr {
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                let new_left = recurse(*left)?;
                let new_right = recurse(*right)?;
                checked_op(op)?
                    .map(
                        |func| quote! { (#new_left).zip(#new_right).and_then(|(l, r)| l.#func(r)) },
                    )
                    .unwrap_or_else(|| quote! { (#new_left).zip(#new_right).map(|(l, r)| l #op r) })
            }
            Expr::Cast(_) => quote! { #expr },
            // Recurse inside the following items
            Expr::Index(_) => quote! { #expr },
            Expr::Lit(ExprLit { lit, .. }) => quote! { Some(#lit) },
            Expr::Paren(ExprParen { expr, .. }) => {
                let new_expr = recurse(*expr)?;
                quote! { ( #new_expr ) }
            }
            Expr::Path(_) => quote! { #expr },
            Expr::Reference(_) => quote! { #expr },
            Expr::Unary(ExprUnary { op, expr, .. }) => {
                let new_expr = recurse(*expr)?;
                quote! { (#new_expr).map(|v| #op v) }
            }
            Expr::Group(ExprGroup { expr, .. }) => recurse(*expr)?,
            _ => bail!("Unexpected expression token {expr:?}"),
        })
    }

    recurse(syn::parse2(item)?)
}

fn checked_op(op: syn::BinOp) -> anyhow::Result<Option<TokenStream>> {
    Ok(match op {
        syn::BinOp::Add(_) => Some(quote! { checked_add }),
        syn::BinOp::Sub(_) => Some(quote! { checked_sub }),
        syn::BinOp::Mul(_) => Some(quote! { checked_mul }),
        syn::BinOp::Div(_) => Some(quote! { checked_div }),
        syn::BinOp::Rem(_) => Some(quote! { checked_rem }),
        syn::BinOp::Shl(_) => Some(quote! { checked_shl }),
        syn::BinOp::Shr(_) => Some(quote! { checked_shr }),
        syn::BinOp::And(_)
        | syn::BinOp::Or(_)
        | syn::BinOp::BitXor(_)
        | syn::BinOp::BitAnd(_)
        | syn::BinOp::BitOr(_)
        | syn::BinOp::Eq(_)
        | syn::BinOp::Lt(_)
        | syn::BinOp::Le(_)
        | syn::BinOp::Ne(_)
        | syn::BinOp::Ge(_)
        | syn::BinOp::Gt(_) => None,
        syn::BinOp::AddAssign(_)
        | syn::BinOp::SubAssign(_)
        | syn::BinOp::MulAssign(_)
        | syn::BinOp::DivAssign(_)
        | syn::BinOp::RemAssign(_)
        | syn::BinOp::BitXorAssign(_)
        | syn::BinOp::BitAndAssign(_)
        | syn::BinOp::BitOrAssign(_)
        | syn::BinOp::ShlAssign(_)
        | syn::BinOp::ShrAssign(_) => bail!("Unsupported operation"),
        _ => bail!("Unknown op: {op:?}"),
    })
}

fn wrapping_op(op: syn::BinOp) -> anyhow::Result<Option<TokenStream>> {
    Ok(match op {
        syn::BinOp::Add(_) => Some(quote! { wrapping_add }),
        syn::BinOp::Sub(_) => Some(quote! { wrapping_sub }),
        syn::BinOp::Mul(_) => Some(quote! { wrapping_mul }),
        syn::BinOp::Div(_) => Some(quote! { wrapping_div }),
        syn::BinOp::Rem(_) => Some(quote! { wrapping_rem }),
        syn::BinOp::Shl(_) => Some(quote! { wrapping_shl }),
        syn::BinOp::Shr(_) => Some(quote! { wrapping_shr }),
        syn::BinOp::And(_)
        | syn::BinOp::Or(_)
        | syn::BinOp::BitXor(_)
        | syn::BinOp::BitAnd(_)
        | syn::BinOp::BitOr(_)
        | syn::BinOp::Eq(_)
        | syn::BinOp::Lt(_)
        | syn::BinOp::Le(_)
        | syn::BinOp::Ne(_)
        | syn::BinOp::Ge(_)
        | syn::BinOp::Gt(_) => None,
        syn::BinOp::AddAssign(_)
        | syn::BinOp::SubAssign(_)
        | syn::BinOp::MulAssign(_)
        | syn::BinOp::DivAssign(_)
        | syn::BinOp::RemAssign(_)
        | syn::BinOp::BitXorAssign(_)
        | syn::BinOp::BitAndAssign(_)
        | syn::BinOp::BitOrAssign(_)
        | syn::BinOp::ShlAssign(_)
        | syn::BinOp::ShrAssign(_) => bail!("Unsupported operation"),
        _ => bail!("Unknown op: {op:?}"),
    })
}

#[cfg(test)]
mod tests;
