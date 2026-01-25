use docs_core::read_file;
use std::path::PathBuf;


fn main() {
    let file_path = PathBuf::from("./testing/main.py").display().to_string();

    let result = read_file(&file_path);

    println!("file contents: {:?}", result);
}
