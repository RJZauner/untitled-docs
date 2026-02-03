//! # HTML Generator module
//!
//! ## Overview
//!
//! This module contains the code to be able to take in
//! a source code file and generate an html
//! file using tera as a templating
//! engine.
//!
use crate::parser::DocsPage;
use tera::Context;
use tera::Tera;

pub fn generate_html(analyser: DocsPage) -> Result<String, Box<dyn std::error::Error>> {
    let tera = match Tera::new("./templates/**/*") {
        Ok(instance) => instance,
        Err(error) => {
            println!("Parsing errors: {}", error);
            ::std::process::exit(1);
        }
    };
    let html = tera.render("pydoc_std.html", &Context::from_serialize(&analyser)?)?;

    Ok(html)
}
