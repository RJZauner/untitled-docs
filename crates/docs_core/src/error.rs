//! # error file for parsing
//!
//! File containing the custom error types for the parser

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse file contents: {message}")]
    ParseError { message: String },
}
