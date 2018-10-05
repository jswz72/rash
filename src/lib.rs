mod commands;
mod outputhandler;
mod config;
mod unixdata;

use std::io::{self, Write, ErrorKind};
use std::error::Error;
use std::env;
use std::process::{Command as PCommand, Stdio};
use std::str;
use outputhandler::OutputHandler;
use config::Config;
use commands::{*};

/// Initialize shell, return new configuraiton object
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
                let command: Command;
                command = parse_input(input);
                if let Command::Exit = command { return }
                execute_command(command, &mut output_handler);
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}

/// Parse given input for commands
fn parse_input<'a>(input: &'a str) -> Command<'a> {
    if input.is_empty() { return Command::Empty }
    if input.contains("|") {
        let pipe_sections = input.split("|");
        let cmds: Vec<Command> = pipe_sections.map(|sect| Command::new(sect.trim())).collect();
        Command::Piped(cmds)
    } else {
        Command::new(input)
    }
    
}

/// Handle given command
    

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
        Some(_) => {
            let mut child = PCommand::new(command)
                .args(args)
                .spawn()
                .expect("failed to start command");
            child.wait()
                .expect("failed to wait for child process");
        }
    }
}

fn execute_pipe(commands: Vec<Command>) -> Result<(), io::Error> {
    if commands.is_empty() { return Ok(())};
    let mut output = vec![];
    let mut stderr = vec![];
    for command in commands {
        if let Command::Program(mut program_command) = command {
            let  ProgramCommand { cmd, mut args, mut flags, ..} = program_command;
            args.append(&mut flags);
            let mut child = PCommand::new(cmd)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            match child.stdin.as_mut() {
                None => return Err(io::Error::new(ErrorKind::Other, "Failed to open stdin")),
                Some(stdin) => stdin.write_all(&output)?,
            };
            let out = child.wait_with_output()?;
            output = out.stdout;
            stderr = out.stderr;
        } else {
            let mut oh = OutputHandler::new();
            match command {
                Command::Ls(file_cmd) => {
                        let FileCommand { mut files, flags, .. } = file_cmd;
                        let add_in = str::from_utf8(&output).unwrap().trim();
                        files.push(add_in);
                        let cmd = Command::Ls(FileCommand { files, flags, output: (commands::Output::Standard, commands::Output::Standard) });
                        cmd.execute(&mut oh)?;
                },
                _ => ()
            }
            output = oh.stdout().as_bytes().to_vec();
            stderr = oh.stderr().as_bytes().to_vec();

        }
    }
    println!("{}", str::from_utf8(&stderr).unwrap());
    println!("{}", str::from_utf8(&output).unwrap());
    Ok(())
}

fn execute_command(command: Command, oh: &mut OutputHandler) {
    match command {
        Command::Empty => (),
        Command::Program(ProgramCommand{ cmd, mut flags, mut args, .. }) => {
            args.append(&mut flags);
            execute_program(cmd, args);
        },
        Command::Piped(pipe_sections) => {
            if let Err(err) = execute_pipe(pipe_sections) {
                oh.add_stderr(err.description());
            }
        },
        _ => {
            if let Err(err) =  command.execute(oh) {
            oh.add_stderr(&format!("{}", err));
            }
            oh.display();
            oh.clear();
        },
    }
}
