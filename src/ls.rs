use std::path::PathBuf;
use std::fs;
use std::env;
use std::io;

pub fn execute(flags: Vec<&str>, args: Vec<&str>) -> Result<(String, String), io::Error>{
    let mut std_err = String::new();
    let mut std_out = String::new();
    let cur_dir = env::current_dir()?;
    let mut paths = vec![cur_dir.clone()];
    for arg in args {
        let mut path = cur_dir.clone();
        path.push(PathBuf::from(arg));
        paths.push(path);
    }
    for path in paths {
        match fs::metadata(path.clone()) {
            Err(err) => std_err = format!("{}\nCannot access {}", std_err, path.to_str().unwrap()),
            Ok(meta) => {
                if !meta.is_dir() {
                    std_err = format!("{}\n{} is not a directory", std_err, path.to_str().unwrap());
                } else {
                    std_out = format!("{}\n{}: ", std_out, path.to_str().unwrap());
                    let dir_contents = fs::read_dir(path.as_path())?;
                    for name in dir_contents {
                        let name = name?.file_name().into_string();
                        match name {
                            Err(_) => std_err = format!("{}\n Directory contains non-unicode filename", std_err),
                            Ok(fname) => std_out = format!("{}\n{}", std_out, fname),
                        }
                    }
                    std_out.push('\n');
                }
            }
        }
    }
    Ok((std_out, std_err))
}
