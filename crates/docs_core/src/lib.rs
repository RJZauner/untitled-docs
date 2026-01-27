//! # Core Processing Library
//!
//! This module defines our core processing library for our docs generation project.

use std::fs;
use std::error::Error;
use rustpython_parser::{parse, Mode};
use rustpython_ast::{Mod};

pub fn read_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    return fs::read_to_string(file_path).map_err(Into::into);
}

pub fn generate_ast(file_contents: &str) -> Result<Mod, Box<dyn Error>> {
    return parse(file_contents, Mode::Module, "<embedded>").map_err(Into::into);
}

pub fn generate_docs() {
    // takes in the ast
    // generates html files
}

