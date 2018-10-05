use outputhandler::OutputHandler;
use commands;
use std::io;

/// Standard output directed to stdout/err, file output for redirection
pub enum Output<'a> {
    Standard,
    File(&'a str)
}

/// Command that calls build in program with optional flag
pub struct BasicCommand<'a> {
    pub flags: Vec<&'a str>,
    pub output: (Output<'a>, Output<'a>)
}

/// Command that calls build in program with optional flag and file
pub struct FileCommand<'a> {
    pub flags: Vec<&'a str>,
    pub files: Vec<&'a str>, 
    pub output: (Output<'a>, Output<'a>)
}

/// Command that calls program in path
pub struct ProgramCommand<'a> {
    pub cmd: &'a str,
    pub flags: Vec<&'a str>,
    pub args: Vec<&'a str>,
    pub output: (Output<'a>, Output<'a>)
}

///Holds valid build-in, external, and piped commands... or Empty
pub enum Command<'a> {
    Ls(FileCommand<'a>),
    Pwd(BasicCommand<'a>),
    Cat(FileCommand<'a>),
    Exit,
    Program(ProgramCommand<'a>),
    Piped(Vec<Command<'a>>),
    Empty,
}

impl<'a> Command<'a> {
    /// Parse input for appropriate command
    pub fn new(input: &'a str) -> Command {
        let output = get_output_type(input);
        let mut input = input.split(' ');
        if let Some(command) = input.next() {
            let is_flag = |i: &&str| i.starts_with("-");
            let input_args = input.clone();

            let flags = input.filter(is_flag).collect();
            let other_tokens = input_args.filter(|i| !is_flag(i)).collect();
            match command {
                "ls" => Command::Ls(FileCommand { flags, files: other_tokens, output }),
                "pwd" => Command::Pwd(BasicCommand { flags, output }),
                "cat" => Command::Cat(FileCommand { flags, files: other_tokens, output }),
                "exit" => Command::Exit,
                _ => Command::Program( ProgramCommand { cmd: command, flags, args: other_tokens, output }),
            }
        } else {
            Command::Empty
        }
    }
    /// Execute built-in shell commands
    pub fn execute_builtin<'b>(&self, oh: &'b mut OutputHandler) -> Result<&'b mut OutputHandler, io::Error> {
        match self {
            Command::Ls(file_cmd) => commands::ls::execute(oh, file_cmd),
            Command::Cat(file_cmd) => commands::cat::execute(oh, file_cmd),
            Command::Pwd(basic_cmd) => commands::pwd::execute(oh, basic_cmd),
            _ => Ok(oh)
        }
    }
}

fn get_output_type(input: &str) -> (Output, Output) {
    let out = Output::Standard;
    let err_out = Output::Standard;
    let mut input = input.split(' ');
    let mut takefile = false;
    for i in input {
        if takefile {
            let file = i;
            return (Output::File(i), Output::Standard);
        }
        if let Some(redirect_idx) = i.find(|x: char| x == '>') {
            if i.len() == 1 {
                takefile = true;
            } else {

            }
        }
    }
    /*let redirect = input.find(|&x| x.contains('>'));
    if let Some(rd) = redirect {
        if rd.len() == 1 {

        }
    }*/
    (Output::Standard, Output::Standard)
}
