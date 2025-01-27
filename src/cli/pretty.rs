use std::fmt::Display;

use owo_colors::OwoColorize;

pub fn error(err: impl Display) {
    println!("{} {}", "error".red().bold(), err);
}

pub fn warning(msg: impl Display) {
    println!("{} {}", "warning".yellow().bold(), msg);
}

pub fn info(msg: impl Display) {
    println!("{} {}", "info".blue().bold(), msg);
}

pub fn msg(title: impl Display, msg: impl Display) {
    println!("{} {}", title.purple().bold(), msg);
}
