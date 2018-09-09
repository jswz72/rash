use std::fs;
use std::str;

const RCFILE: &str = ".rushrc";
const SEPARATOR1: char = '@';
const SEPARATOR2: char = '>';
const END: char = '!';
const USER: &str = "[user]";
const HOST: &str = "[HOST]";
const CWD: &str = "[CWD]";
const FALLBACK: &str = ">";

struct ConfValues {
    enabled: bool,
    value: char,
}

impl ConfValues {
    fn new(enabled: bool, value: char) -> ConfValues {
        ConfValues { enabled, value }
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
    pub fn new() -> Config {
        let mut config_file = fs::read_to_string(RCFILE);
        let mut config = match config_file {
            Err(_) => { 
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
            },
            Ok(ref cf_string) => {
                parse_config_file(&cf_string[..])
            }
        };
        config.prompt = generate_prompt(&mut config);
        config
    }
    pub fn prompt(&self) -> &str {
        &self.prompt
    }
}

fn parse_config_file(config: &str) -> Config {
    let user = config.contains("user=false");
    let separator_pattern = "separator1=";
    let sep_false = format!("{}false", separator_pattern);
    let separator1 = {
        let enabled = config.contains(&sep_false);
        let value = if enabled {
            match get_token(config, separator_pattern) {
                Some(sep) => sep,
                None => SEPARATOR1
            }
        } else { 
            SEPARATOR1 
        };
        ConfValues { enabled, value }
    };
    let host = config.contains("host=false");
    let separator_pattern = "separator2=";
    let sep_false = format!("{}false", separator_pattern);
    let separator2 = {
        let enabled = config.contains(&sep_false);
        let value = if enabled {
            match get_token(config, separator_pattern) {
                Some(sep) => sep,
                None => SEPARATOR2
            }
        } else { 
            SEPARATOR2 
        };
        ConfValues { enabled, value }
    };
    let cwd = config.contains("cwd=false");
    let separator_pattern = "end=";
    let sep_false = format!("{}false", separator_pattern);
    let end = {
        let enabled = config.contains(&sep_false);
        let value = if enabled {
            match get_token(config, separator_pattern) {
                Some(sep) => sep,
                None => END
            }
        } else { 
            END 
        };
        ConfValues { enabled, value }
    };
    Config { prompt: String::from(""), separator1, separator2, end, user, host, cwd }
}

fn get_token(config: &str, sep_pattern: &str) -> Option<char> {
    if !config.contains(sep_pattern) { return None }
    let index = config.find(sep_pattern);
    if let None = index { return None }
    let index = index.unwrap();
    let length = sep_pattern.len();
    let separator_slice = &config[index..index + length];
    separator_slice.chars().last()
}

fn generate_prompt(config: &mut Config) -> String {
    let mut prompt = String::new();
    if config.user {
        prompt = format!("{}{}", prompt, get_user());
    }
    if config.separator1.enabled {
        prompt.push(config.separator1.value);
    }
    if config.host {
        prompt = format!("{}{}", prompt, get_host());
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

fn get_user() -> String { String::from("asdf") }
fn get_host() -> String { String::from("host") }
fn get_cwd() -> String { String::from("cwd") }
