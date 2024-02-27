// /// 
// pub struct VssCommitment(Vec<K256Point>);

// pub struct ShardFeldman {
//     id: u16,
//     poly: Vec<BigInt>, // len == t, poly[0] == x
// }

// impl Deref for VssCommitment {
//     type Target = Vec<K256Point>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl ShardFeldman {
//     pub fn new(
//         id: u16,  // shard id
//         t: u16,   // threshold / quorum. Minimum number of shards to reconstruct the secret.
//         n: u16,   // Total number of shards.
//     ) -> Self {
//         let x_bn = BigInt::from_be_bytes(&x.to_bytes());
//         let K256 = K256Scalar::random(&mut OsRng);
//         // let mut poly: Vec<BigInt> = vec![K256];
//         Self { id, p, poly }
//     }

//     pub fn commit(&self, id: usize) 
// }

// use std::ops::Deref;

// use k256::{elliptic_curve::Field, AffinePoint as K256Point, Scalar as K256Scalar};
// use num_bigint::BigInt;
// use num_traits::FromBytes;
// use rand::rngs::OsRng;

pub mod interop_util;

// #[cfg(test)]
// mod tests;