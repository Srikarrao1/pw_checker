// TODO: Rename this file to change the name of this method from METHOD_NAME

#![no_main]
// #![no_std]  // std support is experimental, but you can remove this to try it

use risc0_zkvm::{
    guest::env,
    sha::{ Impl, Sha256}
};

risc0_zkvm::guest::entry!(main);

pub fn main() {

    let pw: String = env::read();

    let mut is_ok = false;
    for ch in pw.chars() {
        if ch.is_ascii_punctuation() {
            is_ok = true;
        }
    }
    if !is_ok {
        panic!()
    }

    // let digest = sha::digest_u8_slice(pw.as_bytes());
    let t = Impl::hash_bytes(&pw.as_bytes());
    
    env::commit(&t);

    // TODO: Implement your guest code here


}

   