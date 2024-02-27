/// `r == a*x + b*y`
///
/// Particularly, if `r==1`, then
/// * `a*x == 1 (mod b)`, and
/// * `b*y == 1 (mod a)`.
///
pub struct ExtendedEuclideanObject {
    pub r: BigInt, // k-th remainder
    pub x: BigInt, // k-th Bezout coefficient
    pub y: BigInt, // k-th Bezout coefficient
}

/// `gcd = a * bezout_x + b * bezout_y`
///
/// Particularly, if `gcd==1`, then
/// * `a * bezout_x == 1 (mod b)`, and
/// * `b * bezout_y == 1 (mod a)`.
///
/// Also, `a == gcd * reduced_a` and `b == gcd * reduced_b`.
///
#[derive(Debug, Clone, PartialEq /*Symmetric+Transitive*/, Eq /*+Reflective*/)]
pub struct ExtendedEuclideanResult {
    pub gcd: BigInt,
    pub bezout_x: BigInt,
    pub bezout_y: BigInt,
    pub reduced_a: BigInt, // a == gcd * reduced_a
    pub reduced_b: BigInt, // b == gcd * reduced_b
}

pub fn algo_extended_euclidean(a: &BigInt, b: &BigInt) -> ExtendedEuclideanResult {
    let mut prev = ExtendedEuclideanObject {
        r: a.clone(),
        x: const_1(),
        y: const_0(),
    }; // r0 == a == 1*a + 0*b
    let mut curr = ExtendedEuclideanObject {
        r: b.clone(),
        x: const_0(),
        y: const_1(),
    }; // r1 == b == 0*a + 1*b

    while curr.r != const_0() {
        let (q, r) = prev.r.div_rem_euclid(&curr.r);
        let x = &prev.x - &q * &curr.x;
        let y = &prev.y - &q * &curr.y;
        prev = curr;
        curr = ExtendedEuclideanObject { r, x, y };
    }
    if a.is_negative() != curr.y.is_negative() {
        curr.y = -curr.y;
    }
    if b.is_negative() != curr.x.is_negative() {
        curr.x = -curr.x;
    }
    ExtendedEuclideanResult {
        gcd: prev.r,
        bezout_x: prev.x,
        bezout_y: prev.y,
        reduced_a: curr.y,
        reduced_b: curr.x,
    }
}

pub fn moddiv(a: &BigInt, b: &BigInt, p: &BigInt) -> BigInt {
    let obj_ab = algo_extended_euclidean(a, b);
    let a = obj_ab.reduced_a;
    let b = obj_ab.reduced_b;
    if b.is_one() {
        return a.clone();
    }
    let obj_bp = algo_extended_euclidean(&b, &p);
    let b_inv = obj_bp.bezout_x;
    let prod = (a * b_inv).rem_euclid(p);
    prod
}

pub fn modinv(a: &BigInt, p: &BigInt) -> BigInt {
    assert!(false == p.is_zero());
    let obj = algo_extended_euclidean(a, p);
    assert!(obj.gcd.is_one());
    obj.bezout_x.rem_euclid(p)
}

#[cfg(test)] // cargo test -- --show-output
mod tests {
    #[test]
    fn test_extended_euclidean() {
        let a = BigInt::from(288);
        let b = BigInt::from(396);
        let obj = algo_extended_euclidean(&a, &b);
        let gt = ExtendedEuclideanResult {
            gcd: BigInt::from(36),
            bezout_x: BigInt::from(-4),
            bezout_y: BigInt::from(3),
            reduced_a: BigInt::from(8),
            reduced_b: BigInt::from(11),
        };
        assert!(obj == gt);
    }

    #[test]
    fn test_moddiv() {
        let a = BigInt::from(28);
        let b = BigInt::from(8);
        let p = BigInt::from(17);
        let res = moddiv(&a, &b, &p);
        assert_eq!(res, BigInt::from(12));
        let a = BigInt::from(7); // reduce a by gcd(a, b)
        let b = BigInt::from(2); // reduce b by gcd(a, b)
        let res = moddiv(&a, &b, &p);
        assert_eq!(res, BigInt::from(12));
    }

    #[test]
    fn test_modinv() {
        let a = BigInt::from(-2);
        let p = BigInt::from(17);
        let res = modinv(&a, &p);
        assert_eq!(res, BigInt::from(8));
    }

    use super::*;
    use crate::prelude::*;
}

use crate::prelude::*;
