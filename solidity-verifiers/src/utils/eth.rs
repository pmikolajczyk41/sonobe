//! This module provides a trait and implementations for converting Rust types
//! to EVM calldata.

use alloy_primitives::U256;
use ark_ec::{
    pairing::Pairing,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr, CurveGroup,
};
use ark_ff::{BigInteger, Fp, Fp2, Fp2Config, FpConfig, PrimeField};
use ark_groth16::Proof;

pub trait Soliditify {
    type Output;
    fn soliditify(&self) -> Self::Output;
}

impl<T: Soliditify> Soliditify for [T] {
    type Output = Vec<T::Output>;
    fn soliditify(&self) -> Self::Output {
        self.iter().map(Soliditify::soliditify).collect()
    }
}

impl<P: FpConfig<4>> Soliditify for Fp<P, 4> {
    type Output = U256;
    fn soliditify(&self) -> Self::Output {
        let limbs = self.into_bigint().0;
        U256::from_limbs(limbs)
    }
}

impl<P: Fp2Config<Fp: Soliditify>> Soliditify for Fp2<P> {
    type Output = [U256; 2];
    fn soliditify(&self) -> Self::Output {
        [self.c1.soliditify(), self.c0.soliditify()]
    }
}

impl<P: SWCurveConfig<BaseField: Soliditify>> Soliditify for Affine<P> {
    type Output = [U256; 2];
    fn soliditify(&self) -> Self::Output {
        // the encoding of the additive identity is [0, 0] on the EVM
        let (x, y) = self.xy().unwrap_or_default();
        [x.soliditify(), y.soliditify()]
    }
}

impl<P: SWCurveConfig<BaseField: Soliditify>> Soliditify for Projective<P> {
    type Output = [U256; 2];
    fn soliditify(&self) -> Self::Output {
        self.into_affine().soliditify()
    }
}

impl<E: Pairing<G1Affine: Soliditify, G2Affine: Soliditify>> Soliditify for Proof<E> {
    type Output = [[U256; 2]; 3];
    fn soliditify(&self) -> Self::Output {
        [
            self.a.soliditify(),
            self.b.soliditify(),
            self.c.soliditify(),
        ]
    }
}
