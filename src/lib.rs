#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate base64;
extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;

use rand::Rng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, SecretKey, PublicKey, Signature};

use wasm_bindgen::prelude::*;

struct JSRng;

impl Rng for JSRng {
    fn next_u32(&mut self) -> u32 {
        rand()
    }
}

#[wasm_bindgen(module = "./utils")]
extern {
    fn rand() -> u32;
}

#[wasm_bindgen]
pub fn generate_keypair() -> String {
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut JSRng);
    let public = keypair.public.to_bytes();
    let secret = keypair.secret.to_bytes();

    let public_out = base64::encode(&public);
    let secret_out = base64::encode(&secret);

    format!("{}|{}", public_out, secret_out)
}

#[wasm_bindgen]
pub fn sign(message: &str, secret_key: &str) -> String {
    let secret_bytes = base64::decode(secret_key).unwrap();
    let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
    let public = PublicKey::from_secret::<Sha512>(&secret);
    let keypair = Keypair {
        secret, public
    };
    let sig = keypair.sign::<Sha512>(message.as_bytes());
    let sig = sig.to_bytes();

    base64::encode(&sig[..])
}

#[wasm_bindgen]
pub fn verify(message: &str, signature: &str, public_key: &str) -> bool {
    let signature_bytes = base64::decode(signature).unwrap();
    let public_bytes = base64::decode(public_key).unwrap();

    let public = PublicKey::from_bytes(&public_bytes).unwrap();
    let signature = Signature::from_bytes(&signature_bytes).unwrap();

    public.verify::<Sha512>(message.as_bytes(), &signature)
}
