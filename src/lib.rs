mod commands;
mod outputhandler;
mod config;

use std::io::{self, Write};
use outputhandler::OutputHandler;
use config::Config;

///Holds all valid commands or none
#[derive(Debug)]
enum Command<'a> {
    Ls {
        flags: Vec<&'a str>,
        args: Vec<&'a str>,
    },
    Pwd {
        flags: Vec<&'a str>
    },
    Cat {
        flags: Vec<&'a str>,
        args: Vec<&'a str>,
    },
    Exit,
    None
}

pub fn initialize() -> Config {
    Config::new()
}

///Main read-parse-execute loop
pub fn shell_loop(config: Config) {
    let mut output_handler = OutputHandler::new();
    loop {
        print!("{} ", config.prompt());
        io::stdout().flush().expect("Error outputting text");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut input = input.trim();
                let command = parse_input(input);
                if let Command::Exit = command { 
                    break; 
                } else if let Command::None = command {
                    continue
                }
                if let Err(err) =  handle_command(&mut output_handler, command) {
                    output_handler.add_stderr(format!("{}", err));
                }
                output_handler.display();
                output_handler.clear();
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}

/// Parse given input for commands
fn parse_input(input: &str) -> Command {
    let mut input = input.split(' ');
    if let Some(command) = input.next() {
        let is_flag = |i: &&str| i.starts_with("-");
        let input_args = input.clone();

        let flags = input.filter(is_flag).collect();
        let args = input_args.filter(|i| !is_flag(i)).collect();
        match command {
            "ls" => Command::Ls { flags, args, },
            "pwd" => Command::Pwd { flags, },
            "cat" => Command::Cat { flags, args },
            "exit" => Command::Exit,
            _ => Command::None,
        }
    } else {
        Command::None
    }

    
}

/// Handle given command
fn handle_command<'a>(oh: &'a mut OutputHandler, command: Command) -> Result<&'a mut OutputHandler, io::Error> {
    match command {
        Command::Ls{ flags, args } => commands::ls::execute(oh, flags, args),
        Command::Cat{ flags, args } => commands::cat::execute(oh, flags, args),
        Command::Pwd{ flags }=> commands::pwd::execute(oh, flags),
        _ => Ok(oh)
    }
}

pub fn shutdown() {
    println!("goodbye");
}

