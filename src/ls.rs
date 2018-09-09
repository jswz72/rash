use std::path::PathBuf;
use std::fs;
use std::env;
use std::io;
use std::fmt::Write;

pub fn execute(flags: Vec<&str>, args: Vec<&str>) -> Result<(String, String), io::Error>{
    let mut stderr = String::new();
    let mut stdout = String::new();
    let paths = construct_paths(args)?;
    for path in paths {
        match fs::metadata(path.clone()) {
            Err(_) => write!(&mut stderr, "\nCannot access {}", path.to_str().unwrap()).unwrap(),
            Ok(meta) => {
                if !meta.is_dir() {
                    write!(&mut stdout, "\n{} is not a directory", path.to_str().unwrap());
                } else {
                    write!(&mut stdout, "\n{}: ", path.to_str().unwrap());
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

fn construct_paths(paths: Vec<&str>) -> Result<Vec<PathBuf>, io::Error> {
    let cur_dir = env::current_dir()?;
    let home_dir = env::home_dir().unwrap();
    if paths.len() == 0 {
        return Ok(vec![PathBuf::from(cur_dir)]);
    }
    let result: Vec<PathBuf> = paths.iter().map(|path| {
        let mut path_builder = if path.starts_with("~") {
            home_dir.clone()
        } else {
            cur_dir.clone()
        };
        if path.starts_with("./") {
            let (_, trimpath) = path.split_at(path.find('/').unwrap() + 1);
            path_builder.push(PathBuf::from(trimpath))
        } else if path.starts_with("~") {
            let offset = if path.starts_with("~/") { 2 } else { 1 };
            let (_, trimpath) = path.split_at(path.find('~').unwrap() + offset);
            path_builder.push(PathBuf::from(trimpath))
        } else {
            path_builder.push(PathBuf::from(path))
        }
        path_builder
    }).collect();
    Ok(result)
}
