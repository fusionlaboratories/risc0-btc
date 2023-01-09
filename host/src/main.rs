use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use risc0_zkvm::{
    Prover, ProverOpts,
};

use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    // Make the prover.
    let method_code = std::fs::read(MULTIPLY_PATH)
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    let mut rng = rand::thread_rng();
    loop {
        let n: usize = rng.gen_range(0..10_000_000) as usize;
        let start = Instant::now();
        let opts = ProverOpts::default().with_skip_seal(false);
        let mut prover = Prover::new_with_opts(&method_code, MULTIPLY_ID, opts).expect(
            "Prover should be constructed from valid method source code and corresponding method ID",
        );

        prover.add_input_u8_slice(&vec![0; n]);

        // Run prover & generate receipt
        let receipt = prover.run()
            .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");
        println!("{n} {} # {:?}", start.elapsed().as_secs_f64(), receipt.journal);
    }

    // // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    // receipt.verify(MULTIPLY_ID).expect(
    //     "Code you have proven should successfully verify; did you specify the correct method ID?",
    // );
}
