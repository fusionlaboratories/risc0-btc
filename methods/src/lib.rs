#![no_std]

pub mod bench;

// I think, this should prevent recursion, but maybe it doesn't?
#[cfg(not(target_os = "zkvm"))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));
