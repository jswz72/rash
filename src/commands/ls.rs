use commands::utils;

use outputhandler::OutputHandler;
use std::fs;
use std::io;
use commands::FileCommand;

pub fn execute<'a>(oh: &'a mut OutputHandler, command: &FileCommand) -> Result<&'a mut OutputHandler, io::Error>{
    let files = &command.files;
    let paths = utils::construct_paths(files.to_vec())?;
    for path in paths {
        match fs::metadata(path.clone()) {
            Err(_) => oh.add_stderr(&format!("Cannot access {}", path.to_str().unwrap())),
            Ok(meta) => {
                if !meta.is_dir() {
                    oh.add_stderr(&format!("{} is not a directory", path.to_str().unwrap()));
                } else {
                    oh.add_stdout(&format!("{}: ", path.to_str().unwrap()));
                    let dir_contents = fs::read_dir(path.as_path())?;
                    for name in dir_contents {
                        let name = name?.file_name().into_string();
                        match name {
                            Err(_) => oh.add_stderr("Directory contains non-unicode filename"),
                            Ok(fname) => oh.add_stdout(&fname),
                        }
                    }
                    oh.add_stdout("\n");
                }
            }
        }
    }
    Ok(oh)
}


