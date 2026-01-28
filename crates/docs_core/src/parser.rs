//! # Main entry point of parser module
//!
//! This contains all of the methods that allow us to parse python files.

use crate::error::ParserError;
use rustpython_ast::Mod;
use rustpython_parser::{Mode, parse};
use std::fs;
use std::path::Path;

/// Method for returning file contents
///
/// A convenience method that opens and return the file
/// contents.
pub fn read_file_contents<P: AsRef<Path>>(file_path: P) -> Result<String, ParserError> {
    let path = file_path.as_ref();

    // The "?" is fine here - ParserError implement the I/O error
    // therefore this will result in the correct error being
    // propogated.
    let file_contents = fs::read_to_string(path)?;

    // Wrap in "Ok()" to match Result return type.
    Ok(file_contents)
}

/// Parse a single python file as a module
///
/// This is a convenience method for parsing
/// files instead of entire modules.
pub fn parse_module(source: &str) -> Result<Mod, ParserError> {
    return parse(&source, Mode::Module, "<embedded>").map_err(|error| ParserError::ParseError {
        message: error.to_string(),
    });
}

/// Parse a python expression
///
/// A convenience method for parsing a python
/// expression.
pub fn parse_expression(source: &str) -> Result<Mod, ParserError> {
    return parse(&source, Mode::Expression, "<embedded").map_err(|error| {
        ParserError::ParseError {
            message: error.to_string(),
        }
    });
}
