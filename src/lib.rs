mod ls;
mod pwd;
mod cat;

use std::io::{self, Write};

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

pub fn load_config() {
    println!("loading config");
}

///Main read-parse-execute loop
pub fn shell_loop() {
    loop {
        print!("> ");
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
                handle_command(command);

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
            "ls" => Command::Ls {
                flags,
                args,
            },
            "pwd" => Command::Pwd {
                flags,
            },
            "cat" => Command::Cat {
                flags,
                args
            },
            "exit" => Command::Exit,
            _ => Command::None,
        }
    } else {
        Command::None
    }

    
}

/// Hanlde given command
fn handle_command(command: Command) {
    match command {
        Command::Ls{ flags, args } => ls::execute(flags, args),
        Command::Cat{ flags, args } => cat::execute(flags, args),
        Command::Pwd{ flags }=> pwd::execute(flags),
        _ => ()
    }
}

pub fn shutdown() {
    println!("goodbye");
}

