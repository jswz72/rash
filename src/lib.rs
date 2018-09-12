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

trait Pipeable {
    fn add_input(&mut self, input: &str) { }
}

struct BasicCommand<'a> {
    flags: Vec<&'a str>,
}

struct FileCommand<'a> {
    flags: Vec<&'a str>, files: Vec<&'a str>
}

struct ProgramCommand<'a> {
    cmd: &'a str, flags: Vec<&'a str>, args: Vec<&'a str>
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

impl<'b> Pipeable for FileCommand<'b> {
    fn add_input(&mut self, input: &str) {
        self.files.push(input);
    }
}

impl<'a> Pipeable for ProgramCommand<'a> {
    fn add_input(&mut self, input: &str) {
        self.args.push(input);
    }
}

impl<'a> Command<'a> {
    fn add_input(&mut self, input: &str) {
        match self {
            Command::Ls(FileCommand) | Command::Cat(FileCommand) => FileCommand.add_input(input),
            Command::Program(ProgramCommand) => ProgramCommand.add_input(input),
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
    let commands = commands.iter();

    let mut output = vec![];
    let mut commands = commands.peekable();
    while let Some(mut command) = commands.next() {
        if let Command::Program(ProgramCommand {cmd, ref mut args, ref mut flags }) = command {
            args.append(&mut flags);
            let child = PCommand::new(cmd)
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
            output = out.stdout
        } else {
            let mut output_handler = OutputHandler::new();
            command.add_input(str::from_utf8(&output).unwrap());
            handle_command(&mut output_handler, *command);
            if commands.peek().is_some() {
                output_handler.display()
            } else {
                output = output_handler.stdout().as_bytes().to_vec();
            }
        }
    }
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
