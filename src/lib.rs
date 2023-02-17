#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order curve Eddy255. The main feature of this
//! curve is that it embeds the Ed25519 curve, enabling right-field arithmetic in
//! proving arithmetic of Ed25519/EDDSA signatures in a zkSNARK.
//!
//!
//! Curve information:
//! * Base field: q =
//!   57896044618658097711785492504343953926225696987256860989792804023844074237167
//! * Scalar field: r =
//!   57896044618658097711785492504343953926634992332820282019728792003956564819949
//! * Curve equation: y^2 = x^3 + 31257679751294767726203335542381905569456340380844946531343019374821596229325*x + 40361771284572583964927969700930680951746224013842818163834336483522497112790

#[cfg(feature = "curve")]
mod curves;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;
#[cfg(any(feature = "scalar_field", feature = "base_field"))]
pub use fields::*;
