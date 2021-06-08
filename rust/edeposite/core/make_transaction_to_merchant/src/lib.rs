use definitions::*;
use functions::*;
use functions_crypto::*;
use json::object;
use indexmap::*;

pub fn main() {
    let  mut response = object!{}; 
    let _get = _get();
    let merchant_public_address = _get["merchant_pub"].as_str().unwrap();
    let user_public_address = _get["user_pub"].as_str().unwrap();
    let transaction_token = _get["token"].as_str().unwrap().parse::<f32>().unwrap();
    let created_at = timestamp().to_string();

    let _merchant = merchant(merchant_public_address);
    let _user = merchant(user_public_address);

    if !is_merchant(user_public_address){
        response["panic"] = format!("The user public address address '{}' is invalid",user_public_address).into();
    }else if !is_merchant(merchant_public_address){
        response["panic"] = format!("The merchant public address address '{}' is invalid",merchant_public_address).into();
    }else{

        let merchant_initial_token = _merchant["token"].as_f32().unwrap();
        let merchant_final_token = merchant_initial_token - transaction_token;

        let user_initial_token = _user["token"].as_f32().unwrap();
        let user_final_token = user_initial_token + transaction_token;

        let _user_tokens_summary = user_tokens_summary();
        let initial_total_token_in_system = _user_tokens_summary["overall_total"].as_f32().unwrap();
        let final_total_token_in_system = initial_total_token_in_system + transaction_token;
        

        if transaction_token > merchant_initial_token{
            response["panic"] = format!("Transaction token '{}' is greater than merchant '{}' token balance: '{}'",transaction_token,merchant_public_address,merchant_initial_token).into();
        }else{

            let _crypto_conn_pool = crypto_conn_pool();
            let transaction_id =  db::unique_digits_from_db("transactions", "transaction_id", 16, &_crypto_conn_pool);

            response["transaction_token"] = transaction_token.into();
            response["from"] = merchant_public_address.into();
            response["to"] = user_public_address.into();
            response["transaction_id"] = transaction_id.clone().into();
            response["merchant_initial_token"] = merchant_initial_token.into();
            response["merchant_final_token"] = merchant_final_token.into();
            response["user_initial_token"] = user_initial_token.into();
            response["user_final_token"] = user_final_token.into();
            response["initial_total_token_in_system"] = initial_total_token_in_system.into();
            response["final_total_token_in_system"] = final_total_token_in_system.into();
            response["panic"] = json::Null;

            for node_id in active_node_ids(){
                let _settings = node_settings(&node_id);
                let conn_pool = node_conn_pool(&node_id);

                db::execute("UPDATE crypto_merchants SET token=? WHERE public_address=?", vec![merchant_final_token.to_string(),_t(merchant_public_address)], &conn_pool);

                db::execute("UPDATE crypto_merchants SET token=? WHERE public_address=?", vec![user_final_token.to_string(),_t(user_public_address)], &conn_pool);

                let _transaction_id: &str = &transaction_id;
                let _transaction_token: &str = &transaction_token.to_string();


                db::rowaction(DBParametersWithColumns{
                    conn_pool: &conn_pool,
                    table_name: "transactions",
                    columns: indexmap!{
                        "transaction_id"=>_transaction_id,
                        "token"=> _transaction_token,
                        "from"=> merchant_public_address,
                        "to"=> &user_public_address,
                        "to_whom"=>"merchant",
                        "created_at" => &created_at
                    }
                }).insert();

            }
        }
    }
    println!("{}",json::stringify_pretty(response, 4));
}
