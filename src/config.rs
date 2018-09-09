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

struct ConfValues = {
    enabled: bool,
    value: char,
}

pub struct Config {
    prompt: String,
    separator1: ConfValues,
    separator2: ConfValues,
    end: ConfValues,
    user: ConfValues,
    host: ConfValues,
    cwd: ConfValues,
}

impl Config {
    pub fn new() -> Config {
        let mut config_file = fs::read_to_string(RCFILE);
        match config_file {
            Err(_) => { 
                Config { 
                    prompt: { ConfValues {
                        enabled: true,
                        value: String::from(USER), 
                    },
                    separator1: { ConfValues {
                        enabled: true,
                        value: SEPARATOR1.to_string(), 
                    }
                    host: { ConfValues {
                        enabled: true,
                        value: String::from(HOST), 
                    }
                    separator2: { ConfValues {
                        enabled: true,
                        value: SEPARATOR2.to_string()
                    }, 
                    prompt: { ConfValues {
                        enabled: true,
                        value: String::from(CWD), 
                    end: { ConfValues {
                        enabled: true,
                        value:END.to_string(
                    }
                }
            },
            Ok(ref cf_string) => {
                let mut config = parse_config_file(&cf_string[..]);
                config.prompt = generate_prompt(&mut config);
                config
            }
        }
    }
    pub fn prompt(&self) -> String {
        let mut prompt = String::new();
        for item in self.prompt.iter() {
            prompt.push_str(item);
        }
        prompt
    }
}

fn parse_config_file(config: &str) -> Config {
    let mut prompt = Vec::new();
    if !config.contains("user=false") {
        prompt.push(String::from("{user}"));
    }
    let separator_pattern = "separator1=";
    let sep_false = format!("{}false", separator_pattern);
    if !config.contains(&sep_false) {
        let separator = match get_token(config, separator_pattern) {
            Some(sep) => sep,
            None => SEPARATOR1
        };
        prompt.push(separator.to_string());
    }
    if !config.contains("host=false") {
        prompt.push(String::from("{host}"));
    }
    let separator_pattern = "separator2=";
    let sep_false = format!("{}false", separator_pattern);
    if !config.contains(&sep_false) {
        let separator = match get_token(config, separator_pattern) {
            Some(sep) => sep,
            None => SEPARATOR2
        };
        prompt.push(separator.to_string());
    }
    if !config.contains("cwd=false") {
        prompt.push(String::from("{cwd}"));
    }
    let end_pattern = "end=";
    let end_false = format!("{}false", end_pattern);
    if !config.contains(&end_false) {
        let end = match get_token(config, end_pattern) {
            Some(token) => token,
            None => END
        };
        prompt.push(end.to_string());
    }
    Config { prompt }
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

fn generate_prompt(&config: Config) -> String {
    String::new();
}
