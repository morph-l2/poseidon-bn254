use sp1_prover::utils::get_cycles;
use sp1_sdk::include_elf;
use sp1_sdk::{utils, SP1Stdin};

const ELF: &[u8] = include_elf!("poseidon-bn254-test-program");

fn main() {
    utils::setup_logger();
    let cycles = get_cycles(ELF, &SP1Stdin::default());
    println!("final cycles: {cycles}");
}
