# Untitled Docs

A PDoc-like basic parser for Python written in Rust. It is not intended as a full replacement project. This project is mainly used to learn more about Rust and the ecosystem and build something fun while doing that.

## PoC

- Parse entire python projects and keep structure from project for navigation
- From each .py file, extract the following information:
  - file name
  - file path
  - Comments: Package-level comments as well as code comments for classes and functions
  - Classes and their data such as attributes and functions
  - Standalone functions and variables
- Static Site
  - Generate html files
  - Generate index html as a ToC with links to all of the other pages
- Basic CLI
  - Command for generating site
