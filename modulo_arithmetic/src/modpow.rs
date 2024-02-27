/// Let $ e = \sum_{i=0}^{n-1} d_i \cdot 2^i \mod p $, then
/// $ y = \prod_{i=0}^{n-1} (b^2^i)^{d_i} \mod p $.
#[allow(non_snake_case)]
pub fn modpow(base: &BigInt, exp: &BigInt, p: &BigInt) -> BigInt {
    let mut base = base.rem_euclid(p);
    let mut exp = exp.clone();
    if exp.is_negative() {
        base = crate::modinv(&base, p);
        exp = -exp;
    }
    let mut y = const_1();
    let zero = const_0();
    fn is_odd(x: &BigInt) -> bool {
        x.bit(0)
    }

    while exp > zero {
        if is_odd(&exp) {
            y = (y * &base).rem_euclid(p);
        }
        exp >>= 1;
        // Let $ B(n) = b^2^n $, then $ B(n+1) = B(n)^2 $.
        base = (&base * &base).rem_euclid(p);
    }
    y
}

use crate::prelude::*;

#[cfg(test)]
mod tests {
    fn modpow_dumb(base: &BigInt, exp: &BigInt, p: &BigInt) -> BigInt {
        let mut base = base.rem_euclid(p);
        let mut exp = exp.clone();
        if exp.is_negative() {
            base = crate::modinv(&base, &p);
            exp = -exp;
        }
        let mut y = const_1();
        let zero = const_0();
        while exp > zero {
            y = (y * &base).rem_euclid(p);
            exp -= 1;
        }
        y
    }

    #[test]
    fn test_modpow_1() {
        let base = BigInt::from(114); // 2*257
        let exp = BigInt::from(514); // -19*2*3
        let p = BigInt::from(1919); // 19*101
        let est = modpow(&base, &exp, &p);
        let ans = modpow_dumb(&base, &exp, &p);
        assert_eq!(est, ans);
    }

    #[test]
    fn test_modpow_2() {
        let base = BigInt::from(-114);
        let exp = BigInt::from(514);
        let p = BigInt::from(1919);
        let est = modpow(&base, &exp, &p);
        let ans = modpow_dumb(&base, &exp, &p);
        assert_eq!(est, ans);
    }

    #[test]
    fn test_modpow_3() {
        let base = BigInt::from(514);
        let exp = BigInt::from(-114);
        let p = BigInt::from(1919);
        let est = modpow(&base, &exp, &p);
        let ans = modpow_dumb(&base, &exp, &p);
        assert_eq!(est, ans);
    }

    use super::*;
}
