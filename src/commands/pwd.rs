use std::io;
use std::env;
use outputhandler::OutputHandler;

pub fn execute<'a>(oh: &'a mut OutputHandler, _flags: Vec<&str>) -> Result<&'a mut OutputHandler, io::Error>{
    let _std_out = String::new();
    let _std_err = String::new();
    let _path = env::current_dir();

    
    Ok(oh)
}
