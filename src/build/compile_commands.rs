use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct CompileDatabase(Vec<CompileCommand>);

impl CompileDatabase {
    pub fn new() -> Self {
        CompileDatabase(Vec::new())
    }

    pub fn push(&mut self, cmd: CompileCommand) {
        self.0.push(cmd);
    }
}

#[derive(Debug, Serialize)]
pub struct CompileCommand {
    /// The working directory of the compilation. All paths specified in the command or file fields must be either absolute or relative to this directory.
    pub directory: String,
    /// The compile command as a single shell-escaped string. Arguments may be shell quoted and escaped following platform conventions, with " and \ being the only special characters. Shell expansion is not supported.
    pub command: String,
    /// The main translation unit source processed by this compilation step.
    /// This is used by tools as the key into the compilation database.
    /// There can be multiple command objects for the same file, for example if the same source file is compiled with different configurations.
    pub file: String,
}
