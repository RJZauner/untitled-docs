use docs_core::ast_from_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let tree = ast_from_file("./testing/main.py")?;
    println!("AST: {:#?}", tree);
    Ok(())
}
