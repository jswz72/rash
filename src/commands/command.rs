pub struct BasicCommand<'a> {
    flags: Vec<&'a str>,
}

pub struct FileCommand<'a> {
    flags: Vec<&'a str>, files: Vec<&'a str>
}

pub struct ProgramCommand<'a> {
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
    Empty,
}

impl<'a> Command<'a> {
    fn new(input: &'a str) -> Command {
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
            Command::Empty
        }
    }
    fn execute<'b>(&self, oh: &'a mut OutputHandler) -> Result<&'b mut OutputHandler, io::Error> {
        match self {
            Command::Ls(file_cmd) => commands::ls::execute(oh, file_cmd),
            Command::Cat(file_cmd) => commands::cat::execute(oh, file_cmd),
            Command::Pwd(basic_cmd) => commands::pwd::execute(oh, basic_cmd),
            _ => Ok(oh)
        }
    }

}