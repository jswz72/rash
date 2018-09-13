use std::io;
use std::env;
use outputhandler::OutputHandler;
use commands::BasicCommand;

pub fn execute<'a>(oh: &'a mut OutputHandler, command: &BasicCommand) -> Result<&'a mut OutputHandler, io::Error>{
    let _std_out = String::new();
    let _std_err = String::new();
    let _path = env::current_dir();

    
    Ok(oh)
}
