use std::io;

pub fn execute(flags: Vec<&str>, args: Vec<&str>) -> Result<(String, String), io::Error>{
    let out = String::from("...");
    Ok((out.clone(), out))
}
