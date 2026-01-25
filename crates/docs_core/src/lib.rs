//! # Core Processing Library
//!
//! This module defines our core processing library for our docs generation project.

use std::fs;
use std::error::Error;

pub fn read_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let file_contents: String = fs::read_to_string(file_path)?;

    return Ok(file_contents);
}



