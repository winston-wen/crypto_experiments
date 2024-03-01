pub trait DiyHash {
    fn diy_hash(&self) -> String;
}

impl DiyHash for &[u8] {
    fn diy_hash(&self) -> String {
        use blake2::{digest::consts::U16, Blake2b, Digest};

        let mut hasher = Blake2b::<U16>::new();
        hasher.update(self);
        let hash = hasher.finalize();
        bs58::encode(hash).into_string()
    }
}

impl DiyHash for Vec<u8> {
    fn diy_hash(&self) -> String {
        self.as_slice().diy_hash()
    }
}

impl DiyHash for String {
    fn diy_hash(&self) -> String {
        self.as_bytes().diy_hash()
    }
}

use k256::{
    elliptic_curve::{group::prime::PrimeCurveAffine, sec1::ToEncodedPoint, Group},
    AffinePoint, ProjectivePoint,
};
impl DiyHash for AffinePoint {
    fn diy_hash(&self) -> String {
        if self.is_identity().into() {
            "ZERO_POINT".to_string()
        } else if self == &AffinePoint::GENERATOR {
            "GENERATOR".to_string()
        } else {
            self.to_encoded_point(true).as_bytes().diy_hash()
        }
    }
}
impl DiyHash for ProjectivePoint {
    fn diy_hash(&self) -> String {
        if self.is_identity().into() {
            "ZERO_POINT".to_string()
        } else if self == &ProjectivePoint::GENERATOR {
            "GENERATOR".to_string()
        } else {
            self.to_encoded_point(true).as_bytes().diy_hash()
        }
    }
}

use k256::Scalar;
impl DiyHash for Scalar {
    fn diy_hash(&self) -> String {
        if self == &Scalar::ZERO {
            "ZERO_SCALAR".to_string()
        } else if self == &Scalar::ONE {
            "ONE_SCALAR".to_string()
        } else {
            self.0.to_string()
        }
    }
}

use num_bigint::BigInt;
impl DiyHash for BigInt {
    fn diy_hash(&self) -> String {
        self.to_str_radix(10)
    }
}
