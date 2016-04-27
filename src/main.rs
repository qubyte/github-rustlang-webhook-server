extern crate github_webhook_message_validator;
extern crate rustc_serialize;

use github_webhook_message_validator::validate;
use rustc_serialize::hex::FromHex;

const SECRET: &'static [u8] = b"some-secret";

fn check_valid(signature: &str, message: &str) {
    let sig = signature.from_hex().unwrap_or(vec!());
    let mes = message.as_bytes();
    
    println!("is valid: {}", validate(SECRET, &sig, mes));
}

fn main() {
    let some_hex = "736d7f9342f2a7d239afa5513a4bb2283e0e1588";
    let message = "blah-blah-blah";
    
    check_valid(some_hex, message);
}
