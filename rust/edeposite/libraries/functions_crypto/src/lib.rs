use definitions::*;
use functions::*;
use variables::*;
use db;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use serde_json;
use glob::glob;
use mysql::*;
use json::{JsonValue,object,array};


pub fn node_settings(node_id: &str)->NodeSettings{
    let settings_json = file_get_contents(&format!("/{}/{}/{}","/edeposite/nodes",node_id,"settings.json"));
    serde_json::from_str(&settings_json).unwrap()
}


pub fn crypto_conn_pool()->Pool{
    let _settings = settings();
    db::conn(&_settings.db_name,&_settings.db_user,&_settings.db_password,&_settings.db_host,DB_PORT)
}


pub fn node_conn_pool(node_id: &str)->Pool{
    let _settings = node_settings(node_id);
    db::conn(&_settings.db.name, &_settings.db.user, &_settings.db.password, &_settings.db_host, DB_PORT)
}


pub fn active_node_ids()->Vec<String>{
    let mut accumulator: Vec<String> = vec![];
    for node_id in node_ids(){
        let _settings = node_settings(&node_id);
        if _settings.active{
            accumulator.push(node_id);
        }
    }
    accumulator
}



pub fn node_ids()->Vec<String>{
    let mut accumulator: Vec<String> = vec![];

    for entry in glob(&format!("{}/{}",document_root(),"nodes/*")).expect("Failed to read glob pattern") {
        accumulator.push(_t(entry.unwrap().file_name().unwrap().to_str().unwrap()));
    }
    accumulator
}




pub fn public_address()->String{
    let salt = random_varchar(b"abcdef", 1);
    format!("{}{}",salt,random_characters(31))
}


pub fn private_key()->String{
    let mut hasher = Sha256::new();
    let nonce = random_digits(7);    
    hasher.input_str(&nonce);
    hasher.result_str()
}


pub fn user(public_address: &str)->JsonValue{
    let conn_pool = crypto_conn_pool();
    let mut _users = db::fetch("SELECT * FROM crypto_users WHERE public_address=?",_tv(vec![public_address]),&conn_pool);
    
    if _users.len()>0{
        let mut _user = _users.remove(0);

        object!{
            //id: _user.take::<i64,_>("id").unwrap(),
            public_address: _user.take::<String,_>("public_address").unwrap(),
            //private_key: _user.take::<String,_>("private_key").unwrap(),
            email_address: _user.take::<String,_>("email_address").unwrap(),
            phone_number: _user.take::<String,_>("phone_number").unwrap(),
            country: _user.take::<String,_>("country").unwrap(),
            token: _user.take::<f32,_>("token").unwrap(),
            bonus: _user.take::<f32,_>("bonus").unwrap(),
            transactions_count: _user.take::<i32,_>("transactions_count").unwrap(),
            referred_by: _user.take::<Option<String>,_>("referred_by").unwrap(),
            created_at: _user.take::<i32,_>("created_at").unwrap(),
            panic: null
        }
    }else{
        object!{
            panic: Some("public address does not exist")
        }
    }
}



pub fn merchant(public_address: &str)->JsonValue{
    let conn_pool = crypto_conn_pool();
    let mut _merchants = db::fetch("SELECT * FROM crypto_merchants WHERE public_address=?",_tv(vec![public_address]),&conn_pool);
    
    if _merchants.len()>0{
        let mut _merchant = _merchants.remove(0);

        object!{
            //id: _user.take::<i64,_>("id").unwrap(),
            public_address: _merchant.take::<String,_>("public_address").unwrap(),
            //private_key: _user.take::<String,_>("private_key").unwrap(),
            user: user(public_address),
            created_at: _merchant.take::<i32,_>("created_at").unwrap(),
            token: _merchant.take::<f32,_>("token").unwrap_or(0.0),
            hierarchy:_merchant.take::<String,_>("hierarchy").unwrap(),
            panic: null
        }
    }else{
        object!{
            panic: Some("public address does not exist")
        }
    }
}



pub fn is_user(public_address: &str)->bool{
    user(public_address)["panic"].is_null()
}



pub fn is_merchant(public_address: &str)->bool{
    merchant(public_address)["panic"].is_null()
}



pub fn users()->JsonValue{
    let conn_pool = crypto_conn_pool();
    let users = db::fetch("SELECT * FROM crypto_users ORDER BY id DESC",vec![],&conn_pool);
    
    let mut accumulator = object!{};
    for mut _user in users{
        let public_address: String = _user.take("public_address").unwrap();
        accumulator[public_address.clone()] = user(&public_address);
    }
    accumulator
}



pub fn merchants()->JsonValue{
    let conn_pool = crypto_conn_pool();
    let merchants = db::fetch("SELECT * FROM crypto_merchants ORDER BY id DESC",vec![],&conn_pool);
    
    let mut accumulator = object!{};
    for mut _merchant in merchants{
        let public_address: String = _merchant.take("public_address").unwrap();
        accumulator[public_address.clone()] = merchant(&public_address);
    }
    accumulator
}




pub fn transaction(transaction_id: &str)->JsonValue{
    let conn_pool = crypto_conn_pool();
    let mut _transactions = db::fetch("SELECT * FROM transactions WHERE transaction_id=?",_tv(vec![transaction_id]),&conn_pool);
    
    if _transactions.len()>0{
        let mut _transaction = _transactions.remove(0);

        object!{
            transaction_number: _transaction.take::<String,_>("transaction_id").unwrap(),
            token: _transaction.take::<f32,_>("token").unwrap(),
            from: _transaction.take::<String,_>("from").unwrap(),
            to: _transaction.take::<String,_>("to").unwrap(),
            to_whom: _transaction.take::<String,_>("to_whom").unwrap(),
            created_at: _transaction.take::<i32,_>("created_at").unwrap(),
            status: false
        }
    }else{
        object!{
            status: false,
            panic: Some("Transaction does not exists")
        }
    }
}




pub fn transactions()->JsonValue{
    let conn_pool = crypto_conn_pool();
    let transactions = db::fetch("SELECT * FROM transactions ORDER BY id DESC",vec![],&conn_pool);
    
    let mut accumulator = object!{};
    for mut _transaction in transactions{
        let transaction_id: String = _transaction.take("transaction_id").unwrap();
        accumulator[transaction_id] = transaction(&transaction_id);
    }
    accumulator
}




pub fn user_transactions(public_address: &str)->JsonValue{
    let conn_pool = crypto_conn_pool();
    let transactions = db::fetch("SELECT * FROM transactions WHERE `to`=? OR `from`=? ORDER BY id DESC",_tv(vec![public_address,public_address]),&conn_pool);
    
    let mut accumulator = array![];
    for mut _transaction in transactions{
        let transaction_id: String = _transaction.take("transaction_id").unwrap();
        accumulator.push(transaction(&transaction_id)).unwrap();
    }
    accumulator
}



pub fn user_tokens_summary()->JsonValue{
    let conn_pool = crypto_conn_pool();
    let total_token: f64 = db::fetch_one("SELECT SUM(`token`) AS total_token FROM crypto_users", vec![], &conn_pool).take("total_token").unwrap_or(0.0);
    let total_bonus: f64 = db::fetch_one("SELECT SUM(`bonus`) AS total_bonus FROM crypto_users", vec![], &conn_pool).take("total_bonus").unwrap_or(0.0);
    let overall_total: f64 = total_token + total_bonus;

    object!{
        "total_bonus": total_bonus as f32,
        "total_token": total_token as f32,
        "overall_total": overall_total as f32
    }
}




pub fn merchant_tokens_summary()->JsonValue{
    let conn_pool = crypto_conn_pool();
    let total_token: f32 = db::fetch_one("SELECT SUM(`token`) AS total_token FROM crypto_merchants", vec![], &conn_pool).take("total_token").unwrap_or(0.0);

    object!{
        "total_token": total_token 
    }
}


pub fn authenticate_user(public_address: &str, private_key_hash: &str)->bool{
    let conn_pool = crypto_conn_pool();
    let user_private_key: String = db::fetch_one("SELECT * FROM crypto_users WHERE public_address=?", vec![_t(public_address)], &conn_pool).take("private_key").unwrap_or(_t(""));

    sha256_hash(user_private_key) == private_key_hash
}