#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyStore {
    pub id: usize,
    pub vss_scheme: VssLocalScheme,
    pub vss_coms: Vec<VssCommitment>,
    pub vss_secret: BigInt,
}

#[allow(dead_code)]
impl KeyStore {
    /// Set `.id` with param. \
    /// Set `.vss_scheme` as a vec of zeros with length `t`. \
    /// Set `.vss_coms` as a grid of identity points with shape`(n+1, t)`.
    pub fn init(id: usize, t: usize, n: usize) -> Self {
        let poly = vec![const_0(); t];
        let vss_scheme = VssLocalScheme::new_from_poly(poly.clone());
        let poly_com = VssCommitment::new_from_vec(vec![AffinePoint::IDENTITY; t]);
        let vss_coms: Vec<VssCommitment> = vec![poly_com; n + 1];
        let vss_secret = const_0();
        Self {
            id,
            vss_scheme,
            vss_coms,
            vss_secret,
        }
    }

    /// Count of members (shares)
    pub fn n(&self) -> usize {
        self.vss_coms.len()
    }

    /// Minimum count of shares to recover the secret
    pub fn t(&self) -> usize {
        self.vss_scheme.t()
    }

    /// Main public key
    pub fn pk(&self) -> ProjectivePoint {
        let mut res = ProjectivePoint::IDENTITY;
        for com in self.vss_coms.iter() {
            let pk_i = &com[0];
            res += pk_i;
        }
        res
    }
}

use crate::{VssCommitment, VssLocalScheme};
use k256::{AffinePoint, ProjectivePoint};
use modulo_arithmetic::prelude::const_0;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
