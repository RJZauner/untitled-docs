use docs_core::read_file;
use docs_core::generate_ast;
use std::path::PathBuf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = PathBuf::from("./testing/main.py").display().to_string();

    let result = read_file(&file_path).map_err(|error| {
        format!("Could not read file contents: {:?}", error)
    })?;
    
    let tree = generate_ast(&result).map_err(|error| {
        format!("Tree could not be generated: {:?}", error)
    })?;

    println!("AST: {:#?}", tree);

    Ok(())
}
