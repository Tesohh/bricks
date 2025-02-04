use std::process::Command;

pub trait CommandExt {
    /// Transforms a Command to a string representation
    /// ```
    /// use bricks::cli::commandext::CommandExt;
    /// let mut cmd = std::process::Command::new("ls");
    /// cmd.arg("-al").arg(".");
    /// assert_eq!(cmd.to_string(), "ls -al .".to_string());
    /// ```
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
