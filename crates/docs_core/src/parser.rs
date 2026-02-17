//! # Main entry point of parser module
//!
//! This contains all of the methods that allow us to parse python files.

use crate::docs::Class;
use crate::docs::Method;
use crate::docs::Page;
use crate::error::ParserError;
use rustpython_ast::Expr;
use rustpython_ast::Mod;
use rustpython_ast::Stmt;
use rustpython_ast::Visitor;
use rustpython_parser::{Mode, parse};
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct DocsPage {
    pub page: Page,
}

/// DocsPage Constructor - pass it a name to
/// initialise data struct
impl DocsPage {
    pub fn new(name: String) -> Self {
        Self {
            page: Page {
                title: name,
                description: None,
                function: Vec::new(),
                classes: Vec::new(),
                variables: Vec::new(),
            },
        }
    }

    pub fn analyse(&mut self, ast: &Mod) {
        if let Mod::Module(m) = ast {
            for stmt in &m.body {
                self.visit_stmt(stmt.clone())
            }
        }
    }
}

// We first write a fn to return docstrings -> use this to get class docstring, file docstring and
// method docstring

impl Visitor for DocsPage {
    fn visit_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::ClassDef(class) => {
                let mut cls = Class {
                    name: class.name.to_string(),
                    attributes: Vec::new(),
                    methods: Vec::new(),
                    doc_string: parse_docstring(&class.body),
                };

                for stmt in &class.body {
                    if let Stmt::FunctionDef(method) = stmt {
                        let class_method = Method {
                            name: method.name.to_string(),
                            doc_string: parse_docstring(&method.body),
                        };
                        cls.methods.push(class_method);
                    }
                }
                // add class to page struct
                self.page.classes.push(cls);
            }
            Stmt::FunctionDef(function) => {
                println!("Function name: {}", function.name.to_string());
            }
            _ => {
                // run into nothing
            }
        }
    }
}

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

pub fn parse_docstring(body: &[Stmt]) -> Option<String> {
    if body.is_empty() {
        return None;
    }

    if let Stmt::Expr(expr_stmt) = &body[0] {
        if let Expr::Constant(constant) = &*expr_stmt.value {
            if let Some(s) = constant.value.as_str() {
                return Some(s.to_string());
            }
        }
    }

    None
}
