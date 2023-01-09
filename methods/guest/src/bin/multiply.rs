#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

use risc0_zkvm::guest::{env};
use risc0_zkvm_platform::io::SENDRECV_CHANNEL_INITIAL_INPUT;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let initial_input: &'static [u8] = env::send_recv(SENDRECV_CHANNEL_INITIAL_INPUT, &[]);
    let answer: u32 = initial_input.iter().map(|x| *x as u32).sum();
    // let answer = initial_input.len();
    env::commit(&answer);
}
