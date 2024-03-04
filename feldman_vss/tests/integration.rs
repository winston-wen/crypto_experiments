//! Integration test for DKG (Distributed Key Generation)
//!   and recovery (retrieval) of the main secret.
mod thread_fn;
use thread_fn::*;

pub const SAMPLE_T: usize = 4;
pub const SAMPLE_N: usize = 7;

#[tokio::test]
async fn integration() {
    /* ===== test DKG ===== */
    let mut handles = vec![];
    // let members: Vec<usize> = (1..=SAMPLE_N).collect();
    let members: Vec<usize> = vec![3, 5, 7, 11, 13, 17, 19];
    for id in members.iter() {
        let mems = members.clone();
        let handle = tokio::spawn(thread_dkg(*id, SAMPLE_T, mems));
        handles.push(handle);
    }
    for h in handles {
        h.await.unwrap();
    }

    /* ===== test sign ===== */
    use rand::{seq::SliceRandom, thread_rng, Rng};
    let mut rng = thread_rng();
    let n_attend: usize = rng.gen_range(SAMPLE_T..=SAMPLE_N);
    let attendants: Vec<usize> = members
        .choose_multiple(&mut rng, n_attend)
        .cloned()
        .collect();

    /* ===== test recovery ===== */
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