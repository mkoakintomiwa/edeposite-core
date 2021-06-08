use functions_crypto::*;
use colored::*;

fn main(){
    for node_id in active_node_ids(){
        let _settings = node_settings(&node_id);
        println!();
        println!("{}: {} * {}: {} * {}: {}","Node ID".magenta(),_settings.node_id.cyan(),"Name".magenta(),_settings.name.cyan(),"Host".magenta(),_settings.host.cyan());
    }
}

