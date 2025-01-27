use std::env;

/// Returns the value of the environment variable CC
/// if it's not found, will return "cc"
pub fn get_compiler() -> String {
    match env::var("CC") {
        Ok(compiler) => compiler,
        Err(_) => "cc".to_string(),
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
