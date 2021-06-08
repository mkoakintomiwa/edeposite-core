//use std::io::{self,Write};
use colored::*;
use functions::*;
use serde_json::json;
fn main() {

    let nodes_dir = &format!("{}/{}",document_root(),"nodes");
    let node_id = unique_characters_from_fs(nodes_dir,11);
    mkdir(&format!("{}/{}",&nodes_dir,node_id));

    let name = get_info("Name of organization");
    let host = get_info("Host (IP Address)");
    let node_base_url = get_info("Node Base URL");
    let rel_dirname =  get_info("URL Relative Directory");
    let node_url = format!("{}{}",node_base_url,rel_dirname);
    let domain = node_base_url.replace("https://", "").replace("http://", "");
    let handshake_auth_key = random_characters(15);
    let db_name = get_info("DB Name");
    let db_user = get_info("DB User");
    let db_password = get_info("DB Password");
    let ssh_password = get_info("Root SSH Password");

    let node_settings = json!({
        "name": name,
        "node_id": node_id,
        "host": host,
        "node_base_url": node_base_url,
        "rel_dirname": rel_dirname,
        "node_url": node_url,
        "domain": domain,
        "handshake_auth_key": handshake_auth_key,
        "active": true,
        "integration_time": timestamp(),
        "db":{
            "name": db_name,
            "user": db_user,
            "password": db_password
        },
        "ssh": {
            "username": "root",
            "password": ssh_password
        }
    });
    
    file_put_contents(&format!("{}/{}/{}",&nodes_dir,node_id,"settings.json"),&json_encode(&node_settings));

    println!();
    println!("{}{}","The new Node ID: ".magenta(),node_id.cyan());
    println!()
}



// use clap::{Arg, App};

// fn main() {
//     let matches = 
//     App::new("Create node")
//         .version("1.0")
//         .author("Akintomiwa Musthofaa Opemipo")
//             .about("Create nodes for e-deposite cryptocurrency")
//             .arg(Arg::with_name("config")
//                 .short("c")
//                 .long("config")
//                 .value_name("FILE")
//                 .help("Sets a custom config file")
//                 .takes_value(true))
//             .get_matches();

//     // Gets a value for config if supplied by user, or defaults to "default.conf"
//     let config = matches.value_of("config").unwrap_or("default.conf");
//     println!("Value for config: {}", config);
// }



// use functions::*;


// fn main(){
//     println!("{}",input("What will you like to say for Allah's blessing on you > "));
// }