use std::io;
use outputhandler::OutputHandler;
use commands::FileCommand;

pub fn execute<'a>(oh: &'a mut OutputHandler, command: &FileCommand) -> Result<&'a mut OutputHandler, io::Error>{
    Ok(oh)
}
