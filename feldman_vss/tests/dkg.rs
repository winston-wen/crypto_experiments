//! Integration test for DKG (Distributed Key Generation).

mod keystore;
use keystore::*;

#[tokio::test]
async fn test_dkg() {
    use glob::glob;
    use tokio::fs::{create_dir_all, remove_file};
    create_dir_all(KEYSTORE_DIR).await.unwrap();
    for entry in glob(&format!("{}/{}", KEYSTORE_DIR, "*.pickle")).unwrap() {
        let path = entry.unwrap();
        remove_file(path).await.unwrap();
    }

    let mut handles = vec![];
    let t = 4;
    let n = 7;

    for id in 1..=n {
        let handle = tokio::spawn(dkg_thread(id, t, n));
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap();
    }
}

async fn dkg_thread(my_id: usize, t: usize, n: usize) {
    use feldman_vss::{sesman::*, VssCommitment, VssLocalScheme};
    use modulo_arithmetic::prelude::const_0;
    use num_bigint::BigInt;
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

        // This simple but effective assertion
        // protects the threshold of main secret key from being stealthily increased
        // by malicioius DKG participants by using polynomials of degree > t.
        // See following link for detail:
        // https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/
        assert_eq!(com.len(), t);

        keystore.vss_coms[i] = com;
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
        let (poly_com, polyval_com) = keystore.vss_coms[i].prepare_to_check_vss_com(my_id, &polyval_ji);
        assert_eq!(poly_com, polyval_com, "VSS share verification failed for {} -> {}", i, my_id);
        vss_secret += &polyval_ji;
    }
    keystore.vss_secret = vss_secret;

    // Save keystore to disk.
    use serde_pickle::SerOptions;
    let keystore_path = format!("{}/{}.pickle", KEYSTORE_DIR, my_id);
    let keystore_pickle = serde_pickle::to_vec(&keystore, SerOptions::default()).unwrap();
    tokio::fs::write(keystore_path, keystore_pickle)
        .await
        .unwrap();
}
