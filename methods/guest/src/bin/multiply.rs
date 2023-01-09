// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

// use core::arch::asm;

// use risc0_zeroio::deserialize::Deserialize;
// use risc0_zkp::core::sha::{testutil::test_sha_impl, Digest, Sha};
// use risc0_zkvm::guest::{env, memory_barrier, sha::Impl as ShaImpl};
// use risc0_zkvm_methods::multi_test::{MultiTestSpec, MultiTestSpecRef};
// use risc0_zkvm_platform::io::SENDRECV_CHANNEL_INITIAL_INPUT;
use risc0_zkvm::guest::{env};
use risc0_zkvm_platform::io::SENDRECV_CHANNEL_INITIAL_INPUT;
use methods::bench::BenchmarkSpec;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // NOTE(Matthias): type is just copied from send_recv's definition.
    let initial_input: &'static [u8] = env::send_recv(SENDRECV_CHANNEL_INITIAL_INPUT, &[]);
    // TODO: Implement your guest code here
    let answer = initial_input.len();
    // env::commit(b"hello world");
    env::commit(&answer);
}
