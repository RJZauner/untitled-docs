use docs_core::{parse_module, read_file_contents};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file_contents("./testing/main.py")?;

    let tree = parse_module(&content)?;

    println!("AST: {:#?}", tree);

    Ok(())
}
