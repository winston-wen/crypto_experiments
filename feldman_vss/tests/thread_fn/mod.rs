use std::sync::OnceLock;

use dashmap::DashMap;

pub static mut DISK: OnceLock<DashMap<usize, Vec<u8>>> = OnceLock::new();

mod thread_dkg;
pub use thread_dkg::*;
mod thread_recover;
pub use thread_recover::*;
mod thread_sign;
pub use thread_sign::*;