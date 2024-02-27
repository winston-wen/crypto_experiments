mod extended_euclidean;
pub use extended_euclidean::*;
mod modpow;
pub use modpow::*;

pub mod prelude {
    pub fn const_0() -> BigInt {
        BigInt::from(0)
    }
    
    pub fn const_1() -> BigInt {
        BigInt::from(1)
    }

    /// 12th Mersenne Prime. 2^127 - 1.
    pub fn const_mersenne12() -> BigInt {
        pow(BigInt::from(2), 127) - const_1()
    }

    pub fn const_secp256k1_order() -> BigInt {        
        let hexstr = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
        let bytes = hex::decode(hexstr).unwrap();
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes)
    }
    
    pub(crate) use num_bigint::BigInt;
    pub(crate) use num_traits::*;
}
