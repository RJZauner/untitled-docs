//! # Core Processing Library
//!
//! This module defines our core processing library for our docs generation project.

use std::fs;
use std::error::Error;
use rustpython_parser::{parse, Mode};

pub fn read_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(file_path)?;

    return Ok(file_contents);
}

pub fn generate_ast(file_contents: &str) {

    let ast = parse(&file_contents, Mode::Module, "<embedded>");

    println!("AST: {:#?}", &ast);
}

pub fn generate_docs() {
    // takes in the ast
    // generates html files
}

