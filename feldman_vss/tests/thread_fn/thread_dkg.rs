pub async fn thread_dkg(my_id: usize, t: usize, members: Vec<usize>) {
    {
        let n = members.len();
        assert!(1 <= t && t <= n);
        assert!(members.contains(&my_id));
    }

    // Generate random polynomial. Note that the constant term is the distributed secret.
    let my_scheme = VssLocalScheme::new(t);

    // Commit the polynomial.
    let my_com = my_scheme.commit();

    // Send commitment to other participants.
    // id 0 is used as "broadcast" address.
    send("vss_com", my_id, 0, &my_com).await;

    // Receive commitments from other participants.
    let mut vss_coms: HashMap<usize, VssCommitment> = HashMap::new();
    for i in members.iter() {
        let com: VssCommitment = recv("vss_com", *i, 0).await;

        // This simple but effective assertion
        // protects the threshold of main secret key from being stealthily increased
        // by malicioius DKG participants by using polynomials of degree > t.
        // See following link for detail:
        // https://blog.trailofbits.com/2024/02/20/breaking-the-shared-key-in-threshold-signature-schemes/
        assert_eq!(com.len(), t);

        vss_coms.insert(*i, com);
    }

    // Send VSS share to other participants.
    for i in members.iter() {
        let polyval_ji = my_scheme.share_to(*i); // j is my_id
        send("vss_share", my_id, *i, &polyval_ji).await;
    }

    // Receive VSS shares from other participants.
    let mut vss_secret = const_0();
    for i in members.iter() {
        let polyval_ji: BigInt = recv("vss_share", *i, my_id).await; // j is my_id
        let (poly_com, polyval_com) =
            vss_coms[i].prepare_to_check_vss_com(my_id, &polyval_ji);
        assert_eq!(
            poly_com, polyval_com,
            "VSS share verification failed for {} -> {}",
            i, my_id
        );
        vss_secret += &polyval_ji;
    }

    // Construct keystore.
    let keystore = KeyStore {
        id: my_id,
        vss_scheme: my_scheme,
        vss_coms,
        vss_secret,
    };

    // Save keystore to "disk".
    let disk = unsafe { super::DISK.get_or_init(DashMap::new) };
    let buf = serde_pickle::to_vec(&keystore, serde_pickle::SerOptions::default()).unwrap();
    disk.insert(my_id, buf);

    // Comparison among serde implementations:
    // * serde_json: Writes Vec<u8> as array of JSON numbers. Stupid!
    // * serde_pickle: Writes Vec<u8> as binary blob. Good!
    // * bincode: Writes Vec<u8> as binary blob. However, I've encountered deserialization failure months ago.
}

use std::collections::HashMap;

use dashmap::DashMap;
use feldman_vss::{sesman::*, KeyStore, VssCommitment, VssLocalScheme};
use modulo_arithmetic::prelude::*;
use num_bigint::BigInt;
