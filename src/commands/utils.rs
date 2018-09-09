use std::path::PathBuf;
use std::env;
use std::io;

pub fn construct_paths(paths: Vec<&str>) -> Result<Vec<PathBuf>, io::Error> {
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
