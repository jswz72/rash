use commands::utils;

use outputhandler::OutputHandler;
use std::fs;
use std::io;

pub fn execute<'a>(oh: &'a mut OutputHandler, _flags: Vec<&str>, files: Vec<&str>) -> Result<&'a mut OutputHandler, io::Error>{
    let paths = utils::construct_paths(files)?;
    for path in paths {
        match fs::metadata(path.clone()) {
            Err(_) => oh.add_stderr(format!("Cannot access {}", path.to_str().unwrap())),
            Ok(meta) => {
                if !meta.is_dir() {
                    oh.add_stderr(format!("{} is not a directory", path.to_str().unwrap()));
                } else {
                    oh.add_stdout(format!("{}: ", path.to_str().unwrap()));
                    let dir_contents = fs::read_dir(path.as_path())?;
                    for name in dir_contents {
                        let name = name?.file_name().into_string();
                        match name {
                            Err(_) => oh.add_stderr_str("Directory contains non-unicode filename"),
                            Ok(fname) => oh.add_stdout(fname),
                        }
                    }
                    oh.add_stdout_str("\n");
                }
            }
        }
    }
    Ok(oh)
}


