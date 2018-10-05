use std::path::PathBuf;
use std::env;
use std::fs;
use unixdata;
use std::collections::HashMap;

/// Default separators
const RCFILE: &str = ".rushrc";
const SEPARATOR1: char = '@';
const SEPARATOR2: char = '>';
const END: char = '!';

/// Struct for internal properties of config values
struct ConfValues {
    enabled: bool,
    value: char,
}

impl ConfValues {
    pub fn new() -> ConfValues {
        ConfValues { enabled: true, value: ' ' }
    }
}

pub struct Config {
    prompt: String,
    separator1: ConfValues,
    separator2: ConfValues,
    end: ConfValues,
    user: bool,
    host: bool,
    cwd: bool,
}



impl Config {
    /// Read configuration from .rc file and populate config object
    pub fn new() -> Config {
        let config_file = fs::read_to_string(RCFILE);
        let mut config = match config_file {
            Err(_) => { Self::default() },
            Ok(ref cf_string) => { parse_config_file(&cf_string[..]) }
        };
        config.prompt = generate_prompt(&mut config);
        config
    }
    /// Return default config without reading .rc file
    fn default() -> Config {
        Config { 
            prompt: String::from(""),
            user: true,
            separator1: ConfValues {
                enabled: true,
                value: SEPARATOR1, 
            },
            host: true,
            separator2: ConfValues {
                enabled: true,
                value: SEPARATOR2,
            }, 
            cwd: true,
            end: ConfValues {
                enabled: true,
                value:END,
            },
        }
    }
    pub fn prompt(&self) -> &str {
        &self.prompt
    }
    pub fn update_cwd(&mut self) {
        if !self.cwd { return };
        generate_prompt(self);
    }
    pub fn update_user(&mut self) {
        if !self.user { return };
        generate_prompt(self);
    }
    pub fn update_host(&mut self) {
        if !self.host { return };
        generate_prompt(self);
    }
}

/// Parse config file for expected values. Return populated config object
fn parse_config_file(config: &str) -> Config {
    let user = !config.contains("user=false");
    let host = !config.contains("host=false");
    let cwd = !config.contains("cwd=false");
    Config { 
        prompt: String::from(""),
        separator1: get_separator(config, "separator1"), 
        separator2: get_separator(config, "separator2"), 
        end: get_separator(config, "end"), 
        user, 
        host, 
        cwd 
    }
}

fn get_separator(config: &str, sep_pattern: &str) -> ConfValues {
    let (enabled, value) = get_token(config, &format!("{}=", sep_pattern));
    match value {
        Some(value) => ConfValues { enabled, value },
        None => ConfValues { enabled, value: SEPARATOR1 },
    }
}
            
/// Get configured token for giver separator pattern. 
/// Returns enabled status and, if enabled, the token
fn get_token(config: &str, sep_pattern: &str) -> (bool, Option<char>) {
    let sep_false = format!("{}false", sep_pattern);
    let enabled = !config.contains(&sep_false);
    if  !enabled { return (enabled, None) };
    match config.find(sep_pattern) {
        Some(index) => {
            let length = sep_pattern.len();
            let separator_slice = &config[index..index + length];
            (enabled, separator_slice.chars().last())
        },
        None => (enabled, None),
    }
}

fn generate_prompt(config: &mut Config) -> String {
    let mut prompt = String::new();
    if config.user {
        prompt = format!("{}{}", prompt, unixdata::get_user());
    }
    if config.separator1.enabled {
        prompt.push(config.separator1.value);
    }
    if config.host {
        prompt = format!("{}{}", prompt, unixdata::get_host());
    }
    if config.separator2.enabled {
        prompt.push(config.separator2.value);
    }
    if config.cwd {
        prompt = format!("{}{}", prompt, get_cwd());
    }
    if config.end.enabled {
        prompt.push(config.end.value);
    }
    prompt
}

fn get_cwd() -> String {
    let path = env::current_dir();
    if let Err(_) = path { return String::from(unixdata::UNKNOWN) }
    let path = path.unwrap();
    match env::home_dir() {
        None => String::from(path.to_str().unwrap()),
        Some(home_dir) => {
            if path.starts_with(home_dir) {
                let mut path = path.iter();
                path.next();
                path.next();
                let path: PathBuf = path.collect();
                let path = path.to_str().unwrap();
                let squiggle = String::from("~/");
                squiggle + path
            } else {
                String::from(path.to_str().unwrap())
            }

        }
    }
}
