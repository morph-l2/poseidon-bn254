use sp1_prover::utils::get_cycles;
use sp1_sdk::{utils, ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    utils::setup_logger();
    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client
        .prove(&pk, &SP1Stdin::default())
        .compressed()
        .run()
        .unwrap();
    println!("final cycles: {:?}", proof.sp1_version.len());
}
