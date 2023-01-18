#![no_main]
// #![no_std]  // std support is experimental, but you can remove this to try it

// extern { fn fib(x: i32) -> i32; }

// use risc0_zkp::core::sha::Sha;
// use risc0_zkvm::core::sha::Sha;
use risc0_zkvm::guest::{env, sha};
use risc0_zkvm_platform::io::SENDRECV_CHANNEL_INITIAL_INPUT;
use risc0_zkp::core::sha::Sha;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let buf: &'static [u8] = env::send_recv(SENDRECV_CHANNEL_INITIAL_INPUT, &[]);
    let v = buf.to_vec();
    parse_head(&v);
    cfg_if::cfg_if! {
        if #[cfg(target_os = "zkvm")] {
            let shax = sha::Impl {};
            let dig = shax.hash_bytes(&buf).as_bytes();
            let dig2 = shax.hash_bytes(&dig).as_bytes();
            env::commit(&dig2);
        } else {
            panic!();
        }
    }
}

use std::io::Read;
use std::mem;
use std::slice;

// Not sure this is the best way to read a C-struct,
// but it works for now.
pub fn parse_head(v: &Vec<u8>) {
    let mut buffer: &[u8] = &v.clone();
    let mut header: Header = unsafe { mem::zeroed() };
    let header_size = mem::size_of::<Header>();
    unsafe {
        let header_slice =
            slice::from_raw_parts_mut(
                &mut header as *mut _ as *mut u8,
                header_size);
        // `read_exact()` comes from `Read` impl for `&[u8]`
        buffer.read_exact(header_slice).unwrap();
    }
    // println!("{:?}", header);
}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
struct Header {
    version: u32,
    previous_hash: [u8; 32],
    tx_root: [u8; 32],
    time: u32,
    target: u32,
    nonce: u32,
}
