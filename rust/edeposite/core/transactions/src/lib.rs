use functions_crypto::*;
use json;

pub fn main() {
    println!("{:?}", json::stringify_pretty(transactions(), 4));
}
