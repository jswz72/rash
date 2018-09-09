use std::io;
use std::env;

pub fn execute(_flags: Vec<&str>) -> Result<(String, String), io::Error>{
    let _std_out = String::new();
    let _std_err = String::new();
    let _path = env::current_dir();

    let out = String::from("");
    
    Ok((out.clone(), out))
}
