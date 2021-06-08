use functions_crypto::*;
use json;

pub fn main() {
    println!("{:?}", json::stringify_pretty(users(), 4));
}
