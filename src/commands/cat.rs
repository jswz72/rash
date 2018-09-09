use std::io;

pub fn execute(_flags: Vec<&str>, _args: Vec<&str>) -> Result<(String, String), io::Error>{
    let out = String::from("...");
    Ok((out.clone(), out))
}
