mod commands;
mod outputhandler;
mod config;
mod unixdata;

use std::io::{self, Write};
use std::env;
use std::process::{Command as PCommand, Stdio};
use std::str;
use outputhandler::OutputHandler;
use config::Config;

struct BasicCommand<'a> {
    flags: Vec<&'a str>,
}

struct FileCommand<'a> {
    flags: Vec<&'a str>, files: Vec<&'a str>
}

impl<'a> FileCommand<'a> {
    fn add_input(&mut self, input: &'a str) {
        self.files.push(input);
    }
}

struct ProgramCommand<'a> {
    cmd: &'a str, flags: Vec<&'a str>, args: Vec<&'a str>
}

impl<'a> ProgramCommand<'a> {
    fn add_input(&mut self, input: &'a str) {
        self.args.push(input);
    }
}

///Holds all valid commands or none
enum Command<'a> {
    Ls(FileCommand<'a>),
    Pwd(BasicCommand<'a>),
    Cat(FileCommand<'a>),
    Exit,
    Program(ProgramCommand<'a>),
    Piped(Vec<Command<'a>>),
    None,
}

impl<'a> Command<'a> {
    fn add_input(&mut self, input: &'a str) {
        match self {
            Command::Cat(ref mut data) | Command::Ls(ref mut data) => data.add_input(input),
            Command::Program(ref mut data) => data.add_input(input),
            _ => ()
        }
    }
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
            "ls" => Command::Ls(FileCommand { flags, files: other_tokens, }),
            "pwd" => Command::Pwd(BasicCommand { flags }),
            "cat" => Command::Cat(FileCommand { flags, files: other_tokens, }),
            "exit" => Command::Exit,
            _ => Command::Program( ProgramCommand { cmd: command, flags, args: other_tokens }),
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
        Command::Ls(FileCommand { flags, files }) => commands::ls::execute(oh, flags, files),
        Command::Cat(FileCommand { flags, files }) => commands::cat::execute(oh, flags, files),
        Command::Pwd(BasicCommand { flags })=> commands::pwd::execute(oh, flags),
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

fn execute_pipe(mut commands: Vec<Command>) {
    if commands.is_empty() { return };
    let mut output = vec![];
    let mut stderr = vec![];
    for command in commands {
        if let Command::Program(mut program_command) = command {
            let  ProgramCommand { cmd, mut args, mut flags } = program_command;
            args.append(&mut flags);
            let mut child = PCommand::new(cmd)
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute piped process");
            {
                let mut stdin = child.stdin.as_mut().expect("failed to open stdin");
                stdin.write_all(&output);
            }
            let out = child.wait_with_output()
                .expect("failed to wait for piped process");
            output = out.stdout;
            stderr = out.stderr;
        } else {
            let mut oh = OutputHandler::new();
            match command {
                Command::Ls(file_cmd) => {
                        let FileCommand { mut files, flags } = file_cmd;
                        let add_in = str::from_utf8(&output).unwrap().trim();
                        files.push(add_in);
                        let cmd = Command::Ls(FileCommand { files, flags });
                        handle_command(&mut oh, cmd);
                },
                _ => ()
            }
            output = oh.stdout().as_bytes().to_vec();
            stderr = oh.stderr().as_bytes().to_vec();

        }
    }
    println!("{}", str::from_utf8(&stderr).unwrap());
    println!("{}", str::from_utf8(&output).unwrap());
}

fn execute_command(command: Command, oh: &mut OutputHandler) {
    match command {
        Command::None => (),
        Command::Program(ProgramCommand{ cmd, mut flags, mut args }) => {
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
