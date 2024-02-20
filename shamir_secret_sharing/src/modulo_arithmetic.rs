//! Notes on "integer division".
//! Formal definition: `(q, r) = div(a, p)`. s.t. `a == p*q + r && abs(r) < p`.
//! Euclidean division: `r >= 0`. Example: Python `//` and `%`.
//! Centered division: `r` can be negative. Example: Rust `/` and `%`.
//!     In this case, there are two quotients satisfying `abs(r) < p`.
//!     Use the quotients with smaller abs value, i.e. closer to 0.
//! To apply Euclidean division in Rust, one has to use `div_euclid` or `rem_euclid`.
//! ================================================================================

/// Evaluate the polynomial `f(x)`, using Qin Jiushao (秦久韶) / Horner's method.
/// Note that `coefs` is ordered by ascending power of `x`.
pub fn eval_polynomial(coefs: &[BigInt], x: &BigInt, p: &BigInt) -> BigInt {
    let mut y: BigInt = num0();
    for coef in coefs.iter().rev() {
        y = (y * x + coef).rem_euclid(p);
    }
    y
}

pub fn divmod(num: &BigInt, den: &BigInt, p: &BigInt) -> BigInt {
    assert!(*p > num0() && den % p != num0());
    assert!(den % p != num0());
    let (inv, _) = extended_euclid(den, p);
    (num * inv).rem_euclid(p)
}

pub fn extended_euclid(a: &BigInt, b: &BigInt) -> (BigInt, BigInt) {
    let (mut a, mut b) = (a.clone(), b.clone());
    let mut x: BigInt = num0();
    let mut x_prev: BigInt = num1();
    let mut y: BigInt = num1();
    let mut y_prev: BigInt = num0();
    while b != num0() {
        let q = a.div_euclid(&b);
        let r = a.rem_euclid(&b);
        (a, b) = (b, r);
        (x, x_prev) = (x_prev - &q * &x, x);
        (y, y_prev) = (y_prev - &q * &y, y);
    }
    (x_prev, y_prev)
}

pub fn num0() -> BigInt { BigInt::from(0) }
pub fn num1() -> BigInt { BigInt::from(1) }

use num_bigint::BigInt;
use num_traits::Euclid;