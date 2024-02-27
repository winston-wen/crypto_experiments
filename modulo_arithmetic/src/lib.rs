mod extended_euclidean;
pub use extended_euclidean::*;

pub mod prelude {
    pub fn const_0() -> BigInt {
        BigInt::from(0)
    }
    
    pub fn const_1() -> BigInt {
        BigInt::from(1)
    }
    
    pub(crate) use num_bigint::BigInt;
    pub(crate) use num_traits::*;
}
