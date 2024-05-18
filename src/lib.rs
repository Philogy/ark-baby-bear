use ark_ff::fields::{Fp64, MontBackend, MontConfig};

#[derive(MontConfig)]
#[modulus = "2013265921"]
#[generator = "31"]
pub struct FrConfig;
pub type Fr = Fp64<MontBackend<FrConfig, 1>>;
