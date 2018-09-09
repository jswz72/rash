use std::io;
use outputhandler::OutputHandler;

pub fn execute<'a>(oh: &'a mut OutputHandler, _flags: Vec<&str>, _args: Vec<&str>) -> Result<&'a mut OutputHandler, io::Error>{
    Ok(oh)
}
