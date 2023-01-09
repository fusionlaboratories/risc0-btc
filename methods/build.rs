use std::{env};
fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap().contains("zkvm") {
        // Guest shouldn't recursively depend on itself.
        return;
    }

    risc0_build::embed_methods();
}
