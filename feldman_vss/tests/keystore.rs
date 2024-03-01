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
    /// Set `.vss_coms` as a vec of empty vec with length `n+1`.
    pub fn init(id: usize, t: usize, n: usize) -> Self {
        let poly = vec![const_0(); t];
        let coms = vec![VssCommitment::new_from_vec(Vec::new()); n + 1];
        Self {
            id,
            vss_scheme: VssLocalScheme::new_from_poly(poly),
            vss_coms: coms,
            vss_secret: const_0(),
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
            res += &com[0];
        }
        res
    }
}

pub const KEYSTORE_DIR: &str = "/tmp/crypto_experiments/feldman_vss";

use feldman_vss::{VssCommitment, VssLocalScheme};
use k256::ProjectivePoint;
use modulo_arithmetic::prelude::const_0;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
