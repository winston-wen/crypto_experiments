//! Interop between k256::Scalar and num_bigint::BigInt.

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
        // Notes on Endianness
        // Big Endian: power is ordered by descending array index
        // Little Endian: power is ordered by ascending array index
        // e.g. 0x123456 as u32
        // Big Endian:    [0x00, 0x12, 0x34, 0x56],
        //   which is interpreted as coeffients of [256^3, 256^2, 256^1, 256^0].
        // Little Endian: [0x56, 0x34, 0x12, 0x00],
        //   which is interpreted as coefficients of [256^0, 256^1, 256^2, 256^3].

        let n = const_secp256k1_order();
        let a = self.rem_euclid(&n);
        let src_buf = a.to_bytes_be().1;
        let dst_buf: [u8; 32];
        if src_buf.len() > 32 {
            let src_buf_truncated = &src_buf[src_buf.len() - 32..];
            dst_buf = src_buf_truncated.try_into().unwrap();
        } else {
            let pad = vec![0u8; 32 - src_buf.len()];
            let src_buf_padded = [pad, src_buf].concat();
            dst_buf = src_buf_padded.try_into().unwrap();
        }
        Scalar::from_bytes_unchecked(&dst_buf)
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
