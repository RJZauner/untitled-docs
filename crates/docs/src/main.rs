use docs_core::read_file;
use docs_core::generate_ast;
use std::path::PathBuf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from("./testing/main.py").display().to_string();

    let result = read_file(&file_path)?;
    
    generate_ast(&result);

    println!("file contents: {:?}", result);

    Ok(())
}
