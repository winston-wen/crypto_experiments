pub async fn thread_recover(my_id: usize, attendants: Vec<usize>) {
    // Load keystore from "disk".
    assert!(attendants.contains(&my_id));
    let disk = unsafe { super::DISK.get_or_init(DashMap::new) };
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

use dashmap::DashMap;
use feldman_vss::{interop::*, sesman::*, KeyStore};
use modulo_arithmetic::{moddiv, prelude::*};
use num_bigint::BigInt;
use num_traits::Euclid;