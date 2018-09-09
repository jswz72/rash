pub struct OutputHandler {
    stdout: String,
    stderr: String,
}

impl OutputHandler {
    pub fn new() -> OutputHandler { 
        OutputHandler { stdout: String::new(), stderr: String::new() }
    }
    pub fn stdout(&self) -> String { self.stdout.clone() }
    pub fn stderr(&self) -> String { self.stderr.clone() }
    pub fn add_stdout(&mut self, output: String) {
        self.stdout = format!("{}\n{}", self.stdout, output);
    }
    pub fn add_stdout_str(&mut self, output: &str) {
        self.stdout = format!("{}\n{}", self.stdout, output);
    }
    pub fn add_stderr(&mut self, output: String) {
        self.stderr = format!("{}\n{}", self.stderr, output);
    }
    pub fn add_stderr_str(&mut self, output: &str) {
        self.stderr = format!("{}\n{}", self.stderr, output);
    }
    pub fn display_stdout(&self) {
        println!("{}", self.stdout);
    }
    pub fn display_stderr(&self) {
        println!("{}", self.stderr);
    }
    pub fn display(&self) {
        self.display_stderr();
        self.display_stdout();
    }
    pub fn clear_stdout(&mut self) {
        self.stdout = String::new();
    }
    pub fn clear_stderr(&mut self) {
        self.stderr = String::new();
    }
    pub fn clear(&mut self) {
        self.clear_stderr();
        self.clear_stdout();
    }
}
