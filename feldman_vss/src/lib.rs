mod util;
use num_traits::Euclid;
pub use util::*;
use util::{hash::DiyHash, interop::BigIntToScalar};

#[derive(Clone, Debug, Deref, DerefMut, Deserialize, Serialize)]
pub struct VssCommitment(Vec<AffinePoint>);

#[derive(Clone, Debug, Deref, DerefMut, Deserialize, Serialize)]
pub struct VssLocalScheme {
    #[deref]
    poly: Vec<BigInt>, // len == t, poly[0] == x
}

impl VssCommitment {
    pub fn new_from_vec(vec: Vec<AffinePoint>) -> Self {
        Self(vec)
    }

    #[allow(non_snake_case)]
    pub fn prepare_to_check_vss_com(
        &self,            // vss commitment previously received from `id`
        id: usize,        // participant id
        polyval: &BigInt, // value of polynomial computed at and received from `id`
    ) -> (String, String) {
        let x = BigInt::from(id);
        let mut xpow = const_1();
        let order = const_secp256k1_order();

        let mut poly_com = ProjectivePoint::IDENTITY;
        for coef_com in self.iter() {
            let xpow_scalar = xpow.to_scalar();
            let term_com = *coef_com * &xpow_scalar;
            poly_com += term_com;
            xpow = (xpow * &x).rem_euclid(&order);
        }

        let polyval_com = ProjectivePoint::GENERATOR * &polyval.to_scalar();

        let left = poly_com.diy_hash();
        let right = polyval_com.diy_hash();
        (left, right)
    }
}

impl VssLocalScheme {
    pub fn new_from_poly(poly: Vec<BigInt>) -> Self {
        Self { poly }
    }

    pub fn new(
        t: usize, // threshold or quorum. Minimum number of shards to reconstruct the secret.
    ) -> Self {
        let mut rng = rand::thread_rng();
        let one = const_1();
        let order = const_secp256k1_order();
        let mut poly = Vec::new();
        for _ in 0..t {
            let coef = rng.gen_bigint_range(&one, &order);
            poly.push(coef);
        }
        Self { poly }
    }

    pub fn t(&self) -> usize {
        self.poly.len()
    }

    /// What is a "commitment"?
    ///
    /// One creates a secret, and show to others that (s)he holds that secret
    /// by leaving a stub, typically the "public counterpart" of the secret, to others.
    /// The stub that proves (s)he holds the secret is called a "commitment".
    ///
    /// The "public counterpart" is uniquely mapped to the secret,
    /// and solving the secret from the public counterpart is mathematically hard.
    #[allow(non_snake_case)]
    pub fn commit(&self) -> VssCommitment {
        use crate::util::interop::*;

        let mut com = VssCommitment(Vec::new());
        let G = ProjectivePoint::GENERATOR;
        for coef in self.poly.iter() {
            let coef_com = G * &coef.to_scalar();
            com.push(coef_com.to_affine());
        }
        com
    }

    /// Share the secret to a participant.
    pub fn share_to(&self, id: usize) -> BigInt {
        use shamir_secret_sharing::eval_polynomial;

        let id = BigInt::from(id);
        let order = const_secp256k1_order();
        let eval = eval_polynomial(&self.poly, &id, &order);
        eval
    }
}

use derive_more::{Deref, DerefMut};
use k256::{AffinePoint, ProjectivePoint};
use modulo_arithmetic::prelude::{const_1, const_secp256k1_order};
use num_bigint::{BigInt, RandBigInt};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_share() {
        let t: usize = 3;
        let n: usize = 5;
        let vss = VssLocalScheme::new(t);
        let com = vss.commit();
        assert_eq!(com.len(), t);

        for id in 1..=n {
            let share = vss.share_to(id);

            let (poly_com, polyval_com) = com.prepare_to_check_vss_com(id, &share);
            assert_eq!(poly_com, polyval_com, "failed at id={}", id);
        }
    }
}
