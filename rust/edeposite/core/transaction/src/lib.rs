use functions_crypto::*;
pub fn main() {
    let transaction_id = std::env::args().nth(2).unwrap();
    println!("{:?}",json::stringify(transaction(&transaction_id)));
}
