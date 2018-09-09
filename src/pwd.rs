use std::io;
use std::env;

pub fn execute(flags: Vec<&str>) -> Result<(String, String), io::Error>{
    let std_out = String::new();
    let std_err = String::new();
    let path = env::current_dir();

    let out = String::from("");
    
    Ok((out.clone(), out))
}
