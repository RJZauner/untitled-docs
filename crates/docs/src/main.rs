use docs_core::generate_html;
use docs_core::{parse_module, parser::DocsPage, read_file_contents};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file_contents("./testing/main.py")?;

    let ast = parse_module(&content)?;

    let mut analyser = DocsPage::new("test".to_string());
    analyser.analyse(&ast);

    let html = generate_html(analyser)?;

    fs::write("./class.html", &html)?;

    Ok(())
}
