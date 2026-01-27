//! # Core Processing Library
//!
//! This module defines our core processing library for our docs generation project.

pub mod error;
pub mod parser;

pub use crate::parser::ast_from_file;
