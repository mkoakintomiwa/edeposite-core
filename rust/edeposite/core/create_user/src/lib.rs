use definitions::*;
use db;
use functions::*;
use functions_crypto::*;
use indexmap::*;
use json::*;

pub fn main(){
    let _get = _get();
    
    let email = _get["email"].as_str().unwrap();
    let phone_number = _get["phone_number"].as_str().unwrap();
    let country = _get["country"].as_str().unwrap();
    let referred_by = _get["referred_by"].as_str().unwrap_or("null");

    let mut response = object!{};
    
    if referred_by!="null" && referred_by!="" && !is_user(referred_by){
        response["panic"] = format!("The referred_by public_address '{}' is invalid",referred_by).into();    
    }else{


        let _public_address = public_address();
        let _private_key = private_key();
        let created_at = timestamp().to_string();

        for node_id in active_node_ids(){
            let _settings = node_settings(&node_id);
            let conn_pool = node_conn_pool(&node_id);


            db::execute(r#"        	
                CREATE TABLE IF NOT EXISTS `crypto_users` (
                    `id` int(30) NOT NULL AUTO_INCREMENT,
                    `public_address` text,
                    `private_key` text,
                    `email_address` text,
                    `phone_number` varchar(20) DEFAULT NULL,
                    `token` float DEFAULT NULL,
                    `bonus` float DEFAULT NULL,
                    `created_at` int(30) DEFAULT NULL,
                    PRIMARY KEY (`id`)
                ) ENGINE=MyISAM AUTO_INCREMENT=7 DEFAULT CHARSET=latin1
            "#, vec![], &conn_pool);

            let mut _columns = indexmap!{
                "email_address" => email,
                "phone_number" => phone_number,
                "country" => country,
                "public_address" => &_public_address,
                "private_key" => &_private_key,
                "token" => "0",
                "bonus" => "0", 
                "transactions_count"=>"0",
                "created_at" => &created_at
            };

            if referred_by!="null" && referred_by!=""{
                _columns.insert("referred_by",referred_by);
            }

            db::rowaction(DBParametersWithColumns{
                conn_pool: &conn_pool,
                table_name: "crypto_users",
                columns: _columns
            }).insert();

        }
    
        response = object!{
            "public_address" : _public_address,
            "private_key": _private_key,
            "created_at": created_at.parse::<i64>().unwrap(),
            "email": email,
            "phone_number": phone_number,
            "country": country,
            "referred_by": if referred_by!="null"{ referred_by.into() } else { Null }
        };
    }

    println!("{}",json::stringify_pretty(response,4));
}
