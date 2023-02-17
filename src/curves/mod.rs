use ark_ec::{
    models::CurveConfig,
    short_weierstrass::{self as sw, SWCurveConfig},
};
use ark_ff::{Field, MontFp};

use crate::{fq::Fq, fr::Fr};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Eddy255Config;

impl CurveConfig for Eddy255Config {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = 1
    const COFACTOR_INV: Fr = Fr::ONE;
}

pub type Affine = sw::Affine<Eddy255Config>;
pub type Projective = sw::Projective<Eddy255Config>;

impl SWCurveConfig for Eddy255Config {
    /// COEFF_A = 31257679751294767726203335542381905569456340380844946531343019374821596229325
    const COEFF_A: Fq =
        MontFp!("31257679751294767726203335542381905569456340380844946531343019374821596229325");

    /// COEFF_B = 40361771284572583964927969700930680951746224013842818163834336483522497112790
    const COEFF_B: Fq =
        MontFp!("40361771284572583964927969700930680951746224013842818163834336483522497112790");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: Affine = Affine::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);
}

/// G_GENERATOR_X = 1
pub const G_GENERATOR_X: Fq = MontFp!("1");

/// G_GENERATOR_Y = 26205640297237664044156386764537012276698772331209634366891987742733502457447
pub const G_GENERATOR_Y: Fq =
    MontFp!("26205640297237664044156386764537012276698772331209634366891987742733502457447");
