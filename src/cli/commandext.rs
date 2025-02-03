use std::process::Command;

pub trait CommandExt {
    fn to_string(&self) -> String;
}

impl CommandExt for Command {
    fn to_string(&self) -> String {
        format!(
            "{} {}",
            self.get_program().to_string_lossy(),
            self.get_args()
                .map(|arg| arg.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
