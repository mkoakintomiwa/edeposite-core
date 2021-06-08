use db;
use functions::*;
use functions_crypto::*;
use json::{self,object};

pub fn main(){
    let _get = json::parse(&base64_arg()).unwrap();
    let created_at = timestamp().to_string();
    let _public_address = _get["pub"].as_str().unwrap();
    let mut _user = user(_public_address);
    let _merchant = merchant(_public_address);
    
    if _user["panic"].is_null(){

        if !_merchant["panic"].is_null(){
            
            for node_id in active_node_ids(){
                let _settings = node_settings(&node_id);
                let conn_pool = node_conn_pool(&node_id);
        
                db::execute("INSERT INTO crypto_merchants (public_address, token, created_at) VALUES (?,?,?)", _tv(vec![_public_address, "0" ,&created_at ]),&conn_pool);
            }
        
            println!("{}",
                json::stringify_pretty(
                    object!{
                        "public_address" : _public_address,
                        "created_at": created_at.parse::<i64>().unwrap(),
                        "panic":"Account creation successfully for testing mode only"  
                    },4
                )
            );
        }else{
            println!("{}",
                json::stringify_pretty(
                    object!{
                        "panic": "Merchant already exist"  
                    },4
                )
            );    
        }
        
    }else{
        println!("{}",
            json::stringify_pretty(
                object!{
                    "panic": _user["panic"].to_string()  
                },4
            )
        );
    }
}
