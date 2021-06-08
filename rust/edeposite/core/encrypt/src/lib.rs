// use openssl::rsa::Rsa;
use functions as fx;
use pem::{Pem, encode};
use sha2::{Sha256, Digest};
use json::{object,JsonValue};
//use openssl::rsa::Rsa;

pub fn main() {
  // Generate a new 4096-bit key.
  //let rsa = Rsa::generate(4096).unwrap();

//   let public_key = rsa.public_key_to_der().unwrap();
//   let private_key = rsa.private_key_to_der().unwrap();

    let _get = fx::_get(); 

    //let rsa = Rsa::generate(4096).unwrap();

    let sender_private_key = _get["sender_private_key"].as_str().unwrap_or("");

    //let private_key_signature = rsa.private_key_to_der().unwrap();

    // create a Sha256 object
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(sender_private_key);

    // read hash digest and consume hasher
    let sender_private_key_hash = format!("{:X}",hasher.finalize());

    let token = _get["token"].as_str().unwrap_or("0");

    let sender_public_address = _get["sender_public_address"].as_str().unwrap_or("");

    let recipient_public_address = _get["recipient_public_address"].as_str().unwrap_or("");

    let signed_by = _get["signed_by"].as_str().unwrap_or("");

    let signatory_percentage_charge = _get["signatory_percentage_charge"].as_str().unwrap_or("");

    let transaction_data = object!{
        token: token,
        sender_public_address: sender_public_address,
        recipient_public_address: recipient_public_address,
        created_at: fx::timestamp(),
        sender_private_key_hash: sender_private_key_hash,
        signed_by: signed_by,
        signatory_percentage_charge: signatory_percentage_charge
        //private_key_signature: private_key_signature
    };

    let mut final_transaction_data = transaction_data.clone();

    let mut transaction_hasher = Sha256::new();

    let transaction_data_string = json::stringify_pretty(transaction_data, 4);

    transaction_hasher.update(transaction_data_string);

    let transaction_hash = format!("{:X}",transaction_hasher.finalize());

    final_transaction_data["hash"] = JsonValue::from(transaction_hash);

    let final_transaction_data_string =  format!("{:#}",final_transaction_data);


    let private_pem = Pem {
        tag: String::from("TRANSACTION SIGNATURE"),
        contents: final_transaction_data_string.into_bytes()
    };
    let transaction_hash_pem = encode(&private_pem);

    println!("{}", transaction_hash_pem);

}


