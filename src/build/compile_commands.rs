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
    pub directory: String,
    pub command: String,
    pub file: String,
}
