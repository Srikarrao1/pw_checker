// TODO: Update the name of the method loaded by the prover. E.g., if the method is `multiply`, replace `METHOD_NAME_ID` with `MULTIPLY_ID` and replace `METHOD_NAME_PATH` with `MULTIPLY_PATH`
use methods::{PW_CHECKER_ID, PW_CHECKER_ELF};
// use risc0_zkvm::host::Prover;
// use risc0_zkp::prove::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use risc0_zkp::core::digest::Digest;

use risc0_zkvm::guest::env;
use risc0_zkvm::{ 
    ExecutorEnv,
    Executor,
    sha::{Impl, Sha256}
};

fn main() {

    let passwd: String = "Hello!23".into();

    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&passwd).unwrap())
        .build();

    let mut exec = Executor::from_elf(env, PW_CHECKER_ELF).unwrap();

    let session = exec.run().unwrap();

    let receipt = session.prove().unwrap();


    let t: Digest = from_slice(&receipt.journal).unwrap();

    println!("result ==============> {:?}", t);


        


    // let mut input_bytes = Vec::<u8>::new();
    // env::stdin().read_to_end(&mut input_bytes).unwrap();

    // Make the prover.
    // let method_code = std::fs::read(PW_CHECKER_PATH).unwrap();
    // println!("method_code =======> {:?}", method_code);
        
    // let mut prover = Prover::new(&method_code.into(), PW_CHECKER_ID).expect(
    //     "Prover should be constructed from valid method source code and corresponding method ID",
    // );

    // let pw = String::from("Gamer!con");

    // prover.add_input(&to_vec(&pw).unwrap()).unwrap();



    // // TODO: Implement communication with the guest here

    // // Run prover & generate receipt
    // let receipt = prover.run()
    //     .expect("Code should be provable unless it 1) had an error or 2) overflowed the cycle limit. See `embed_methods_with_options` for information on adjusting maximum cycle count.");

    // // Optional: Verify receipt to confirm that recipients will also be able to verify your receipt
    // receipt.verify(PW_CHECKER_ID).expect(
    //     "Code you have proven should successfully verify; did you specify the correct method ID?",
    // );

    // // TODO: Implement code for transmitting or serializing the receipt for other parties to verify here

    // let digest: Digest = from_slice(receipt.journal.unwrap()).unwrap();
    // // println!("Digest of journal is {}", hex::encode(digest));
    // println!("we proved that the hash {} came from the digest", digest)
}
