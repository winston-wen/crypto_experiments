//! Integration test for DKG (Distributed Key Generation)
//!   and recovery (retrieval) of the main secret.

use std::sync::OnceLock;

use dashmap::DashMap;
use feldman_vss::{interop::*, sesman::*, KeyStore, VssCommitment, VssLocalScheme};
use modulo_arithmetic::{moddiv, prelude::*};
use num_bigint::BigInt;
use num_traits::Euclid;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub const SAMPLE_T: usize = 4;
pub const SAMPLE_N: usize = 7;
static mut DISK: OnceLock<DashMap<usize, Vec<u8>>> = OnceLock::new();

#[tokio::test]
async fn integration() {
    /* ===== test DKG ===== */
    let mut handles = vec![];
    for id in 1..=SAMPLE_N {
        let handle = tokio::spawn(thread_dkg(id, SAMPLE_T, SAMPLE_N));
        handles.push(handle);
    }
    for h in handles {
        h.await.unwrap();
    }

    /* ===== test recovery ===== */
    let mut rng = thread_rng();
    let members: Vec<usize> = (1..=SAMPLE_N).collect();
    let n_attend: usize = rng.gen_range(SAMPLE_T..=SAMPLE_N);
    let attendants: Vec<usize> = members
        .choose_multiple(&mut rng, n_attend)
        .cloned()
        .collect();
    let mut handles = vec![];
    for id in attendants.iter() {
        let att = attendants.clone();
        let handle = tokio::spawn(thread_recover(*id, att));
        handles.push(handle);
    }
    for h in handles {
        h.await.unwrap();
    }
}

async fn thread_dkg(my_id: usize, t: usize, n: usize) {
    assert!(1 <= my_id && my_id <= n);

    let mut keystore = KeyStore::init(my_id, t, n);

    // Generate random polynomial. Note that the constant term is the distributed secret.
    let vss_scheme = VssLocalScheme::new(t);
    keystore.vss_scheme = vss_scheme;
    let vss_scheme = &keystore.vss_scheme;

    // Commit the polynomial.
    let my_com = vss_scheme.commit();
    keystore.vss_coms[my_id] = my_com;
    let my_com = &keystore.vss_coms[my_id];

    // Send commitment to other participants.
    // id 0 is used as "broadcast" address.
    send("vss_com", my_id, 0, my_com).await;

    // Receive commitments from other participants.
    for i in 1..=n {
        let com: VssCommitment = recv("vss_com", i, 0).await;
        assert_eq!(com.len(), t);
        keystore.vss_coms[i] = com;

        // The above simple but effective assertion
        // protects the threshold of main secret key from being stealthily increased
        // by malicioius DKG participants by using polynomials of degree > t.
        // See following link for detail:
        // https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/
    }

    // Send VSS share to other participants.
    for i in 1..=n {
        let polyval_ji = vss_scheme.share_to(i); // j is my_id
        send("vss_share", my_id, i, &polyval_ji).await;
    }

    // Receive VSS shares from other participants.
    let mut vss_secret = const_0();
    for i in 1..=n {
        let polyval_ji: BigInt = recv("vss_share", i, my_id).await; // j is my_id
        let (poly_com, polyval_com) =
            keystore.vss_coms[i].prepare_to_check_vss_com(my_id, &polyval_ji);
        assert_eq!(
            poly_com, polyval_com,
            "VSS share verification failed for {} -> {}",
            i, my_id
        );
        vss_secret += &polyval_ji;
    }
    keystore.vss_secret = vss_secret;

    // Save keystore to "disk".
    let disk = unsafe { DISK.get_or_init(DashMap::new) };
    let buf = serde_pickle::to_vec(&keystore, serde_pickle::SerOptions::default()).unwrap();
    disk.insert(my_id, buf);

    // Comparison among serde implementations:
    // * serde_json: Writes Vec<u8> as array of JSON numbers. Stupid!
    // * serde_pickle: Writes Vec<u8> as binary blob. Good!
    // * bincode: Writes Vec<u8> as binary blob. However, I've encountered deserialization failure months ago.
}

async fn thread_recover(my_id: usize, attendants: Vec<usize>) {
    // Load keystore from "disk".
    assert!(attendants.contains(&my_id));
    let disk = unsafe { DISK.get_or_init(DashMap::new) };
    let buf = disk.get(&my_id).unwrap();
    let keystore: KeyStore =
        serde_pickle::from_slice(&buf, serde_pickle::DeOptions::default()).unwrap();
    assert_eq!(keystore.id, my_id);

    // Send vss secrets.
    send("vss_secret", my_id, 0, &keystore.vss_secret).await;

    // Receive vss secrets.
    use std::collections::HashMap;
    let mut vss_secrets: HashMap<usize, BigInt> = HashMap::new();
    for i in attendants.iter() {
        let vss_secret: BigInt = recv("vss_secret", *i, 0).await;
        vss_secrets.insert(*i, vss_secret);
    }

    // Recover the main secret key.
    // There is NO way to RECOVER the main secret key
    //   without EXPOSING vss secret to the public network traffic.
    // However, there are mature methods to SIGN with the main secret key
    //   without EXPOSING any vss secret to the public network traffic.
    // One of the methods is GG18.
    let mut main_secret = const_0();
    let order = const_secp256k1_order();
    for i in attendants.iter() {
        let vss_secret = vss_secrets.get(i).unwrap();
        let mut lambda = const_1();
        for j in attendants.iter() {
            if i == j {
                continue;
            }
            let num = BigInt::from(*j);
            let den = BigInt::from(*j) - BigInt::from(*i);
            lambda *= moddiv(&num, &den, &order);
        }
        main_secret += vss_secret * lambda;
    }
    main_secret = main_secret.rem_euclid(&order);

    // Validate against the main public key.
    use k256::ProjectivePoint;
    let sk = main_secret.to_scalar();
    let pk_eval = ProjectivePoint::GENERATOR * sk;
    assert_eq!(pk_eval, keystore.pk());
}
