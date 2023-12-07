use std::{env, fs, path};

/// Gets input file path in format of "{binary-name}.txt" in same directory
pub fn get_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let bin_path = dbg!(path::Path::new(&args[0]));
    let file_name = bin_path.file_name()?.to_str()?;

    Some("src/bin/".to_string() + file_name + ".txt")
}

/// Gets all content of a file in "./src/bin/{binary_name}.txt"
pub fn get_input_text() -> Option<String> {
    let path = get_path()?;
    fs::read_to_string(path).ok()
}

pub fn get_text(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}
