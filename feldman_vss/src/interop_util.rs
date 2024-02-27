pub trait ScalarToBigInt {
    fn to_bigint(&self) -> BigInt;
}
impl ScalarToBigInt for Scalar {
    fn to_bigint(&self) -> BigInt {
        BigInt::from_bytes_be(num_bigint::Sign::Plus, self.to_bytes().as_slice())
    }
}

pub trait BigIntToScalar {
    fn to_scalar(&self) -> Scalar;
}
impl BigIntToScalar for BigInt {
    fn to_scalar(&self) -> Scalar {
        let n = const_secp256k1_order();
        let a = self.rem_euclid(&n);
        let bvec = a.to_bytes_be().1;
        let mut bytes: [u8; 32] = <[u8; 32]>::default();
        bytes.copy_from_slice(&bvec);
        Scalar::from_bytes_unchecked(&bytes)
    }
}

use k256::Scalar;
use modulo_arithmetic::prelude::const_secp256k1_order;
use num_bigint::BigInt;
use num_traits::Euclid;

#[cfg(test)]
mod tests {
    use super::*;
    use k256::{elliptic_curve::Field, Scalar};
    use modulo_arithmetic::{modinv, prelude::const_secp256k1_order};
    use num_bigint::BigInt;
    use num_traits::Euclid;
    use rand::rngs::OsRng;

    #[test]
    fn test_interop() {
        let n = const_secp256k1_order();

        let mut x_sc = Scalar::random(&mut OsRng);
        if Scalar::ZERO == x_sc {
            x_sc += Scalar::ONE;
        }
        let mut x_bn = x_sc.to_bigint();
        let est_x_sc = x_bn.to_scalar();
        assert_eq!(x_sc, est_x_sc);

        x_sc *= &Scalar::from(114u32);
        x_bn = (x_bn * &BigInt::from(114)).rem_euclid(&n);
        let est_x_sc = x_bn.to_scalar();
        assert_eq!(x_sc, est_x_sc);

        x_sc += &Scalar::from(514u32);
        x_bn = (x_bn + &BigInt::from(514)).rem_euclid(&n);
        let est_x_sc = x_bn.to_scalar();
        assert_eq!(x_sc, est_x_sc);

        x_sc = x_sc.invert().unwrap();
        x_bn = modinv(&x_bn, &n);
        let est_x_sc = x_bn.to_scalar();
        assert_eq!(x_sc, est_x_sc);
    }
}
