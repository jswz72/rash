mod commands;
mod outputhandler;
mod config;
mod unixdata;

use std::io::{self, Write};
use std::env;
use outputhandler::OutputHandler;
use config::Config;
use std::process::Command as PCommand;

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
    Other {
        cmd: &'a str,
        flags: Vec<&'a str>,
        args: Vec<&'a str>,
    },
    None,

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
                    continue;
                } else if let Command::Other{ cmd, mut flags, mut args } = command {
                    args.append(&mut flags);
                    execute_subprocess(cmd, args);
                    continue;
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
    if input == "" { return Command::None }
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
            _ => Command::Other { cmd: command, flags, args },
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


/// Execute given command as child process
fn execute_subprocess(command: &str, args: Vec<&str>) {
    let cmd_location = match env::var_os("PATH") {
        Some(paths) => {
            env::split_paths(&paths).filter_map(|dir| {
                let full_path = dir.join(command);
                if full_path.exists() {
                    Some(full_path)
                } else {
                    None
                }
            }).next()
        }
        None => {
            println!("PATH is not defined");
            None
        }
    };
    match cmd_location {
        None => println!("{} is not defined in path", command),
        Some(cmd) => {
            let mut child = PCommand::new(command)
                .args(args)
                .spawn()
                .expect("failed to start command");
            let errcode = child.wait()
                .expect("failed to wait for child process");
        }
    }
    }
