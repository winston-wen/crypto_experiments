#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyStore {
    pub id: usize,
    pub vss_scheme: VssLocalScheme,
    pub vss_coms: HashMap<usize, VssCommitment>,
    pub vss_secret: BigInt,
}

#[allow(dead_code)]
impl KeyStore {
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
        for com in self.vss_coms.values() {
            let pk_i = &com[0];
            res += pk_i;
        }
        res
    }
}

use std::collections::HashMap;

use crate::{VssCommitment, VssLocalScheme};
use k256::ProjectivePoint;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
