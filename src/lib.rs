use std::io::{self, Write};

///Holds all valid commands or none
enum Command<'a> {
    Ls(Vec<&'a str>),
    Pwd,
    Cat(Vec<&'a str>),
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
    let mut command_tokens = input.split(' ');
    if let Some(command) = command_tokens.next() {
        match command {
            "ls" => Command::Ls(command_tokens.collect()),
            "pwd" => Command::Pwd,
            "cat" => Command::Cat(command_tokens.collect()),
            "exit" => Command::Exit,
            _ => Command::None,
        }
    } else {
        Command::None
    }

    
}

/// Execute given command
fn handle_command(command: Command) {
    match command {
        Command::Ls(arr) => println!("{:?}", arr),
        Command::Cat(arr) => println!("{:?}", arr),
        Command::Pwd => println!("pwd"),
        _ => ()
    }
}

pub fn shutdown() {
    println!("goodbye");
}

