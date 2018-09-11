mod commands;
mod outputhandler;
mod config;
mod unixdata;

use std::io::{self, Write};
use std::env;
use outputhandler::OutputHandler;
use config::Config;
use std::process::{Command as PCommand, Stdio};
use std::str;

///Holds all valid commands or none
#[derive(Debug)]
enum Command<'a> {
    Ls { flags: Vec<&'a str>, files: Vec<&'a str> },
    Pwd { flags: Vec<&'a str> },
    Cat { flags: Vec<&'a str>, files: Vec<&'a str> },
    Exit,
    Program { cmd: &'a str, flags: Vec<&'a str>, args: Vec<&'a str> },
    Piped(Vec<Command<'a>>),
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
        io::stdout().flush().expect("Error outputting prompt");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut input = input.trim();
                let command = parse_input(input);
                if let Command::Exit = command { return }
                execute_command(command, &mut output_handler);
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}
fn build_command(input: &str) -> Command {
    let mut input = input.split(' ');
    if let Some(command) = input.next() {
        let is_flag = |i: &&str| i.starts_with("-");
        let input_args = input.clone();

        let flags = input.filter(is_flag).collect();
        let other_tokens = input_args.filter(|i| !is_flag(i)).collect();
        match command {
            "ls" => Command::Ls { flags, files: other_tokens, },
            "pwd" => Command::Pwd { flags },
            "cat" => Command::Cat { flags, files: other_tokens, },
            "exit" => Command::Exit,
            _ => Command::Program { cmd: command, flags, args: other_tokens },
        }
    } else {
        Command::None
    }
}

/// Parse given input for commands
fn parse_input(input: &str) -> Command {
    if input.is_empty() { return Command::None }
    if input.contains("|") {
        let pipe_sections = input.split("|");
        let cmds: Vec<Command> = pipe_sections.map(|sect| build_command(sect.trim())).collect();
        Command::Piped(cmds)
    } else {
        build_command(input)
    }
    
}

/// Handle given command
fn handle_command<'a>(oh: &'a mut OutputHandler, command: Command) -> Result<&'a mut OutputHandler, io::Error> {
    match command {
        Command::Ls{ flags, files } => commands::ls::execute(oh, flags, files),
        Command::Cat{ flags, files } => commands::cat::execute(oh, flags, files),
        Command::Pwd{ flags }=> commands::pwd::execute(oh, flags),
        _ => Ok(oh)
    }
}

pub fn shutdown() {
    println!("goodbye");
}


/// Try to execute given program name as child if found in system PATH
fn execute_program(command: &str, args: Vec<&str>) {
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
            child.wait()
                .expect("failed to wait for child process");
        }
    }
}

fn execute_pipe(commands: Vec<Command>) {
    if let Command::Program {ref cmd, ref args, ref flags} = commands[0] {
        if let Command::Program {cmd: cmd2, flags: ref flags2, args: ref args2} = commands[1] {
            let output = PCommand::new(cmd)
                .args(args)
                .output()
                .expect("failed to execute first piped process");
            let mut child = PCommand::new(cmd2)
                .args(args2)
                .stdin(Stdio::piped())
                .spawn()
                .expect("failed to execute second piped process");
            {
                let mut stdin = child.stdin.as_mut().expect("failed to open stdin");
                stdin.write_all(&output.stdout);
            }
            child.wait();
        }
    }
}

fn execute_command(command: Command, oh: &mut OutputHandler) {
    match command {
        Command::None => (),
        Command::Program{ cmd, mut flags, mut args } => {
            args.append(&mut flags);
            execute_program(cmd, args);
        },
        Command::Piped(pipe_sections) => execute_pipe(pipe_sections),
        _ => {
            if let Err(err) =  handle_command(oh, command) {
            oh.add_stderr(format!("{}", err));
            }
            oh.display();
            oh.clear();
        },
    }
}
