use functions::*;
use functions_crypto::*;
use db;

fn main(){
    println!();
    let command = input("Query> ");
    println!();
    let parameters = explode(&input("Parameter> "),",");
    println!();

    for node_id in active_node_ids(){
        let _settings = node_settings(&node_id);
        let conn_pool = node_conn_pool(&node_id);
        db::execute(&command, parameters.clone(), &conn_pool);
        println!("{} ran query",_settings.name);
        println!("");
    }
}
