use std::fs;

const RCFILE: &str = ".rushrc";
const PROMPT: &str = "{user}@{host}>{cwd}";
const SEPARATOR1: char = '@';
const SEPARATOR2: char = '>';
const END: char = '!';

pub struct Config {
    prompt: String,
}

impl Config {
    pub fn new(&self) -> Config {
        let mut config_file = fs::read_to_string(RCFILE);
        match config_file {
            Err(_) => { Config { prompt: String::from(PROMPT) } },
            Ok(cf) => self.parse_config(cf)
        }
    }
    fn parse_config(&self, config: String) -> Config {
        let mut prompt = String::new();
        if !config.contains("user=false") {
            prompt.push_str("{user}");
        }
        let separator_pattern = "separator1=";
        let sep_false = format!("{}false", separator_pattern);
        if !config.contains(&sep_false) {
            let separator = match Config::get_separator(&config, separator_pattern) {
                Some(sep) => sep,
                None => SEPARATOR1
            };
            prompt.push(separator);
        }
        if !config.contains("host=false") {
            prompt.push_str("{host}");
        }
        let separator_pattern = "separator2=";
        let sep_false = format!("{}false", separator_pattern);
        if !config.contains(&sep_false) {
            let separator = match Config::get_separator(&config, separator_pattern) {
                Some(sep) => sep,
                None => SEPARATOR2
            };
            prompt.push(separator);
        }
        if !config.contains("cwd=false") {
            prompt.push_str("{cwd}");
        }
        Config { prompt }
    }
    fn get_separator(config: &String, sep_pattern: &str) -> Option<char> {
        if !config.contains(sep_pattern) { return None }
        let index = config.find(sep_pattern);
        if let None = index { return None }
        let index = index.unwrap();
        let length = sep_pattern.len();
        let separator_slice = &config[index..index + length];
        separator_slice.chars().last()
    }
}
