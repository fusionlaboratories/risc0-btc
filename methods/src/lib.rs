#![no_std]

pub mod bench;
#[cfg(not(target_os = "zkvm"))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));
