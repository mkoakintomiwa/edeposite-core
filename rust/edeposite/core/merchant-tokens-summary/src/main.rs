use functions_crypto::*;
use json;

fn main() {
    println!("{}",json::stringify_pretty(merchant_tokens_summary(),4));
}
