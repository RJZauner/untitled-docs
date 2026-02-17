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
  - Parses a basic config toml file with paths for python project as well as template directories
- Basic CLI
  - Command for generating site

## Design Changes

- **PageData** - a data struct that handles the data of each parsed python file
- **Parser** - a project parser that takes in the project's directory, builds a Sitemap which is a struct that contains all of the PageData from the python source files and generates the final html files from this Sitemap struct. It returns the Sitemap struct to Site as well as the configured location of the generated html files.
- **Server**: This handles the actual serving of the generated html files
- **Site** - main struct for building site - passes data to other parts, handles configuration as well as errors.

## Next Changes

- Complete parsing engine - take in the python file and generate a complete example site from it - ensure templating works and adapt styling
- Parse a complete directory with Sitemap struct