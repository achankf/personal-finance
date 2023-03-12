use std::path::PathBuf;

pub fn get_default_folder() -> PathBuf {
    let mut path = std::env::current_dir().expect("to be able to open the current directory");
    path.push("private");
    path
}
