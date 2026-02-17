//! # Docs Structs & Enums
//!
//! This file contains the structs and enums for building the
//! static site.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Page {
    pub title: String,
    pub description: Option<String>,
    pub classes: Vec<Class>,
    pub function: Vec<Method>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Serialize)]
pub struct Class {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub methods: Vec<Method>,
    pub doc_string: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Attribute {
    pub name: String,
    pub doc_string: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Method {
    pub name: String,
    pub doc_string: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Variable {
    pub name: String,
}
