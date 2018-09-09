use commands::utils;

use std::fs;
use std::io;
use std::fmt::Write;

pub fn execute(_flags: Vec<&str>, args: Vec<&str>) -> Result<(String, String), io::Error>{
    let mut stderr = String::new();
    let mut stdout = String::new();
    let paths = utils::construct_paths(args)?;
    for path in paths {
        match fs::metadata(path.clone()) {
            Err(_) => write!(&mut stderr, "\nCannot access {}", path.to_str().unwrap()).unwrap(),
            Ok(meta) => {
                if !meta.is_dir() {
                    write!(&mut stdout, "\n{} is not a directory", path.to_str().unwrap()).unwrap();
                } else {
                    write!(&mut stdout, "\n{}: ", path.to_str().unwrap()).unwrap();
                    let dir_contents = fs::read_dir(path.as_path())?;
                    for name in dir_contents {
                        let name = name?.file_name().into_string();
                        match name {
                            Err(_) => stderr.push_str("\n Directory contains non-unicode filename"),
                            Ok(fname) => write!(&mut stdout, "\n{}", fname).unwrap(),
                        }
                    }
                    stdout.push('\n');
                }
            }
        }
    }
    Ok((stdout, stderr))
}


