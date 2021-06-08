use functions as fx;

pub fn main(){
    let mut args = std::env::args();
    println!();
    match args.nth(2).unwrap().as_str(){
        
        "init"=>{
            let remote_address = fx::input("Remote address: ");
            fx::shell_exec(&format!("git init && git remote add origin {}",remote_address));
        }
        
        "push"=>{
            fx::shell_exec(r#"cd /edeposite && git add . --all && git commit -m "Auto Commit" && git branch -M main &&git push -u origin main"#);
            print!("");
        }

        _=>{
            
        }
    }
    println!();
}