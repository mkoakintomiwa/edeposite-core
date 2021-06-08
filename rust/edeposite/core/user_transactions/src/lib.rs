use functions_crypto::*;
pub fn main() {
    let public_address = std::env::args().nth(2).unwrap();
    println!("{:?}",json::stringify(user_transactions(&public_address)));
}
