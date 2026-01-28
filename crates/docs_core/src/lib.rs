//! # This crate is used to generate an abstract syntax tree
//! based on Python source code.
//!
//! ## Overview
//! This module defines our core processing library for our
//! documentation generation.

pub mod error;
pub mod parser;

pub use crate::parser::parse_module;
pub use crate::parser::read_file_contents;
