fn main() {
    let p = pow(BigInt::from(2), 127) - const_1(); // 12th Mersenne Prime. 2^127 - 1.
    let s = BigInt::from(1145141919810893i64);
    let k: usize = 3;
    let n: usize = 5;
    let shares = share_secret(&s, k, n, &p);

    let view = [&shares[0], &shares[1], &shares[2]];
    let es = lagrange_interpolate(&view, &p);
    assert_eq!(es, s);

    let view = [&shares[0], &shares[1], &shares[2], &shares[3]];
    let es = lagrange_interpolate(&view, &p);
    assert_eq!(es, s);

    let view = [&shares[0], &shares[1], &shares[2], &shares[3], &shares[4]];
    let es = lagrange_interpolate(&view, &p);
    assert_eq!(es, s);
}

#[derive(Debug, Clone)]
pub struct Share {
    /// Unique and Non-Zero.
    /// Because id is used as argument `x` for polynomial `f(x)`, and `f(0)` is the secret.
    pub id: usize,

    pub val: BigInt,
}

pub fn share_secret(
    s: &BigInt,
    k: usize, // (quorum) Need >=k shares to reconstruct the secret.
    n: usize, // Total number of shares.
    p: &BigInt,
) -> Vec<Share> {
    assert!(k <= n);
    assert!(*p > const_0());
    let mut coefs: Vec<BigInt> = vec![s.clone()];
    let mut rng = rand::thread_rng();
    for _ in 1..k {
        let coef = rng.gen_bigint_range(&const_1(), &p);
        coefs.push(coef);
    }
    let mut shares: Vec<Share> = Vec::new();
    for id in 1..=n {
        let x = BigInt::from(id);
        let share = Share {
            id,
            val: eval_polynomial(&coefs, &x, p),
        };
        shares.push(share);
    }
    shares
}

/// Evaluate the secret (i.e. `f(0)`) from shares.
/// $$
/// f(0) = \sum_{i=0}^{k-1} {y_i \cdot
///     \prod_{j=0, j \neq i}^{k-1}{\frac{x_j}{x_j - x_i}}
/// }
/// $$
pub fn lagrange_interpolate(shares: &[&Share], p: &BigInt) -> BigInt {
    '_check_uniqueness: {
        let shares_set: HashSet<usize> = shares
            .iter()
            .map(|s| usize::from_be_bytes(s.id.to_be_bytes()[..8].try_into().unwrap()))
            .collect();
        assert_eq!(shares.len(), shares_set.len());
    }
    let mut sum: BigInt = const_0();
    for (i, share) in shares.iter().enumerate() {
        let x_i = BigInt::from(share.id);
        let y_i = &share.val;
        let mut prod_i: BigInt = const_1();
        for (j, other_share) in shares.iter().enumerate() {
            if i == j {
                continue;
            }
            let x_j = BigInt::from(other_share.id);
            let frac = moddiv(&x_j, &(&x_j - &x_i), p);
            prod_i = (prod_i * frac).rem_euclid(p);
        }
        let sum_i = (y_i * prod_i).rem_euclid(p);
        sum = (sum + sum_i).rem_euclid(p);
    }
    sum
}


/// Evaluate the polynomial `f(x)`, using Qin Jiushao (秦久韶) / Horner's method.
/// Note that `coefs` is ordered by ascending power of `x`.
pub fn eval_polynomial(coefs: &[BigInt], x: &BigInt, p: &BigInt) -> BigInt {
    let mut y: BigInt = const_0();
    for coef in coefs.iter().rev() {
        y = (y * x + coef).rem_euclid(p);
    }
    y
}

use modulo_arithmetic::*;
use modulo_arithmetic::prelude::*;
use num_bigint::{BigInt, RandBigInt};
use num_traits::{*, pow};
use std::collections::HashSet;