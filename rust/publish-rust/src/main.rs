use std::process::Command;
use std::path::Path;
use std::env;

fn main() {

    let current_dir = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let application_name = Path::new(&current_dir).file_name().unwrap().to_str().unwrap();

    if cfg!(target_os = "windows") {
    
        let source = format!(r#"{}\target\release\{}.exe"#,current_dir,application_name);
        let destination = format!(r#"{}\..\..\bin\{}.exe"#,current_dir,application_name); 

        Command::new("cmd")
                .args(&["/C", "copy", &source, &destination, "/Y" ])
                .output()
                .expect("failed to execute process")
    } else {
        let source = format!(r#"{}/target/release/{}"#,current_dir,application_name);
        let destination = format!(r#"/bin/{}"#,application_name);

        Command::new("cp")
                .arg(&source).arg(&destination)
                .output()
                .expect("failed to execute process")
    };
    
    println!("{} published",application_name);
}
