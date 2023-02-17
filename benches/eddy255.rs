use ark_algebra_bench_templates::*;
use eddy255::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Eddy255",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
