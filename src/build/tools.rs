use std::env;

use crate::config::brick::BrickLang;

/// Returns the value of the environment variable CC (or CXX if cpp)
/// if it's not found, will return "cc" (or "c++" if cpp)
pub fn get_compiler(lang: BrickLang) -> String {
    match lang {
        BrickLang::C => match env::var("CC") {
            Ok(compiler) => compiler,
            Err(_) => "cc".to_string(),
        },
        BrickLang::CPP => match env::var("CXX") {
            Ok(compiler) => compiler,
            Err(_) => "c++".to_string(),
        },
    }
}

/// Returns the value of the environment variable AR
/// if it's not found, will return "ar"
pub fn get_archiver() -> String {
    match env::var("AR") {
        Ok(archiver) => archiver,
        Err(_) => "ar".to_string(),
    }
}
