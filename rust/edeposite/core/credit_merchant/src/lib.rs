use definitions::*;
use db;
use functions::*;
use functions_crypto::*;
use indexmap::*;
use json::*;

pub fn main(){
    let _get = json::parse(&base64_arg()).unwrap();
    let transaction_id =  db::unique_digits_from_db("transactions", "transaction_id", 16, &crypto_conn_pool());
    let token = _get["token"].as_str().unwrap();
    let _public_address = _get["pub"].as_str().unwrap();
    let auth_key =  _get["auth_key"].as_str().unwrap();
    let approved_auth_key = "eae4c345ad45ea70ca2de";
    let created_at = timestamp().to_string();

    let _merchant = merchant(_public_address);
    
    if _merchant["panic"].is_null(){
        if approved_auth_key==auth_key{
            
            for node_id in active_node_ids(){
                let conn_pool = node_conn_pool(&node_id);
                db::execute("UPDATE crypto_merchants SET token=token+? WHERE public_address=?", _tv(vec![token,_public_address]), &conn_pool);

                db::rowaction(DBParametersWithColumns{
                    conn_pool: &conn_pool,
                    table_name: "transactions",
                    columns: indexmap!{
                        "transaction_id"=>transaction_id.as_str(),
                        "token"=>token,
                        "from"=>"edeposite",
                        "to"=> _public_address,
                        "created_at" => created_at.as_str()
                    }
                }).insert();
            }

            println!("{}",json::stringify_pretty(object!{
                "transaction_number": transaction_id.as_str(),
                "token": token.parse::<f32>().unwrap(),
                "from": "edeposite",
                "to": _public_address,
                "status": "Transaction successful",
                "panic": null          
            }, 4));


        }else{
            println!("{}",json::stringify_pretty(object!{
                "panic": "Wrong authentication key"           
            }, 4));
        }
    }else{
        println!("{}",json::stringify_pretty(object!{
            "panic": format!("Merchant of the public address `{}` does not exist",_public_address)           
        }, 4));
    }
        
    
    //println!("{:?}",d)
}
