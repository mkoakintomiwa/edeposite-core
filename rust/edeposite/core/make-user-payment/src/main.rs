use definitions::*;
use functions::*;
use functions_crypto::*;
use json::object;
use indexmap::*;

fn main() {
    let  mut response = object!{}; 
    let _get = _get();
    let sender_address = _get["sender_address"].as_str().unwrap();
    let sender_private_key = _get["sender_private_key"].as_str().unwrap();
    let recipient_address = _get["recipient_address"].as_str().unwrap();
    let transaction_token = _get["token"].as_str().unwrap().parse::<f32>().unwrap();
    let merchant_address = _get["merchant_address"].as_str().unwrap();
    let merchant_transaction_charge =  _get["merchant_transaction_charge"].as_str().unwrap().parse::<f32>().unwrap();
    let merchant_transaction_token = merchant_transaction_charge/100. * transaction_token;

    let edeposite_address = "aFgFHNWe21y4D8JWESE8cPiF2FNdBoFg";
    let edeposite_transaction_charge = 0.01;
    let edeposite_transaction_token = edeposite_transaction_charge/100. * transaction_token;

    let transaction_token_with_charge = transaction_token + merchant_transaction_token + edeposite_transaction_token;

    let created_at = timestamp().to_string();
    

    let sender = user(sender_address);
    let recipient = user(recipient_address);


    if !is_user(sender_address){
        response["panic"] = format!("The user public address address '{}' is invalid",sender_address).into();
    }else if !is_user(recipient_address){
        response["panic"] = format!("The merchant public address address '{}' is invalid",recipient_address).into();
    }else if !authenticate_user(sender_address,sender_private_key){
        response["panic"] = "Private key authentication failed".into();
    }else{

        let sender_initial_token = sender["token"].as_f32().unwrap();
        let sender_final_token = sender_initial_token - transaction_token_with_charge;

        let recipient_initial_token = recipient["token"].as_f32().unwrap();
        let recipient_final_token = recipient_initial_token + transaction_token;

        let recipient_tokens_summary = user_tokens_summary();
        let initial_total_token_in_system = recipient_tokens_summary["overall_total"].as_f32().unwrap();
        let final_total_token_in_system = initial_total_token_in_system + transaction_token_with_charge;
       
        let recipient_transactions_count = recipient["transactions_count"].as_i32().unwrap();
        

        if transaction_token_with_charge > sender_initial_token{
            response["panic"] = format!("Transaction token '{}eDT' is greater than user '{}' balance: '{}eDT'",transaction_token_with_charge,sender_address,sender_initial_token).into();
        }else{

            let _crypto_conn_pool = crypto_conn_pool();
            let transaction_id =  db::unique_digits_from_db("transactions", "transaction_id", 16, &_crypto_conn_pool);

            response["transaction_token"] = transaction_token.into();
            response["total_transaction_charge"] = (merchant_transaction_charge + edeposite_transaction_charge).into();
            response["total_transaction_token_charge"] = (merchant_transaction_token + edeposite_transaction_token).into(); 
            response["from"] = sender_address.into();
            response["to"] = recipient_address.into();
            response["transaction_id"] = transaction_id.clone().into();
            response["sender_initial_token"] = sender_initial_token.into();
            response["sender_final_token"] = sender_final_token.into();
            response["recipient_initial_token"] = recipient_initial_token.into();
            response["receipient_final_token"] = recipient_final_token.into();
            // response["initial_total_token_in_system"] = initial_total_token_in_system.into();
            // response["final_total_token_in_system"] = final_total_token_in_system.into();
            response["panic"] = json::Null;

            for node_id in active_node_ids(){
                let _settings = node_settings(&node_id);
                let conn_pool = node_conn_pool(&node_id);

                db::execute("UPDATE crypto_users SET token=? WHERE public_address=?", vec![sender_final_token.to_string(),_t(sender_address)], &conn_pool);
                
                db::execute("UPDATE crypto_users SET token=?,transactions_count=transactions_count+1 WHERE public_address=?", vec![recipient_final_token.to_string(),_t(recipient_address)], &conn_pool);

                db::execute("UPDATE crypto_users SET token=token+? WHERE public_address=?", vec![edeposite_transaction_token.to_string(),_t(edeposite_address)], &conn_pool);

                db::execute("UPDATE crypto_users SET token=token+? WHERE public_address=?", vec![merchant_transaction_token.to_string(),_t(merchant_address)], &conn_pool);


                if recipient_transactions_count==0{
                    for (dividend_recipient_address,dividend_recipient) in users().entries(){
                        let dividend_recipient_initial_token = dividend_recipient["token"].as_f32().unwrap();
                        let dividend_bonus: f32;
                        let dividend_recipient_bonus: f32;

                        if dividend_recipient["public_address"].as_str().unwrap() != recipient_address{
                            if recipient["referred_by"].is_null(){
                                dividend_bonus = (10.0/100.0) * transaction_token;
                                dividend_recipient_bonus = (dividend_recipient_initial_token/final_total_token_in_system) * dividend_bonus;
        
                            }else{
                                let referred_by_public_address = recipient["referred_by"].as_str().unwrap();
                                //let referred_by_user = user(referred_by_public_address);
                                if dividend_recipient["public_address"].as_str().unwrap() != referred_by_public_address{
                                    dividend_bonus = (5.0/100.0) * transaction_token;
                                    dividend_recipient_bonus = (dividend_recipient_initial_token/final_total_token_in_system) * dividend_bonus;
                                }else{
                                    dividend_recipient_bonus = (5.0/100.0) * transaction_token;
                                    
                                }
                            }
                            db::execute("UPDATE crypto_users SET bonus=bonus+? WHERE public_address=?", _tv(vec![&dividend_recipient_bonus.to_string(),dividend_recipient_address]), &conn_pool);
                        }

                    }
                }


                let _transaction_id: &str = &transaction_id;
                let _transaction_token: &str = &transaction_token.to_string();


                db::rowaction(DBParametersWithColumns{
                    conn_pool: &conn_pool,
                    table_name: "transactions",
                    columns: indexmap!{
                        "transaction_id"=>_transaction_id,
                        "token"=> _transaction_token,
                        "from"=> sender_address,
                        "to"=> &recipient_address,
                        "created_at" => &created_at
                    }
                }).insert();

            }

            // response["calculations"] = object!{
            //     "recipient_final_token": "recipient_initial_token + transaction_token",
            //     "sender_final_token": "sender_initial_token - transaction_token",
            //     "final_total_token_in_system":"initial_total_token_in_system + transaction_token",
            //     "not_referred_dividend_bonus":"(10/100) X transaction_token",
            //     "referred_dividend_bonus":"(5/100) X transaction_token",
            //     "dividend_recipient_bonus":"(dividend_recipient_initial_token/final_total_token_in_system) X dividend_bonus",
            //     "referred_by_recipient_bonus":"(5/100) X transaction_token"
            // };
        }
    }
    println!("{}",json::stringify_pretty(response, 4));
}
