use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::{
    Prover, ProverOpts,
};

use std::time::{Instant};
use std::fs;

fn main() {
    let head = fs::read("block80.raw").unwrap();
    parse_head(&head);

    let method_code = std::fs::read(MULTIPLY_PATH).unwrap();
    let start = Instant::now();
    let opts = ProverOpts::default().with_skip_seal(false);
    let mut prover = Prover::new_with_opts(&method_code, MULTIPLY_ID, opts).unwrap();

    prover.add_input_u8_slice(&head);

    let receipt = prover.run().unwrap();
    println!("{} # {:?}", start.elapsed().as_secs_f64(), receipt.journal);

    // // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    // receipt.verify(MULTIPLY_ID).expect(
    //     "Code you have proven should successfully verify; did you specify the correct method ID?",
    // );
}

use std::io::Read;
use std::mem;
use std::slice;

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
    println!("{:?}", header);
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
