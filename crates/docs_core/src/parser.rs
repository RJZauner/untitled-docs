//! # Main entry point of parser module
//!
//! This contains all of the methods that allow us to parse python files.

use rustpython_ast::Mod;
use rustpython_parser::{Mode, parse};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn ast_from_file<P: AsRef<Path>>(file_path: P) -> Result<Mod, Box<dyn Error>> {
    let path = file_path.as_ref();

    let file_contents = fs::read_to_string(path)?;

    return parse(&file_contents, Mode::Module, "<embedded>").map_err(Into::into);
}
