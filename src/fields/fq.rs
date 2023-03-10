use ark_ff::fields::{Fp256, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "57896044618658097711785492504343953926225696987256860989792804023844074237167"]
#[generator = "2"]
pub struct FqConfig;
pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
