//! Java AST Parser — a syn-style API for Java 25.
//!
//! This crate provides a complete Java Abstract Syntax Tree parser that closely
//! follows the Java Language Specification (JLS) SE 25. The API design is inspired
//! by Rust's [`syn`] crate.
//!
//! # Quick Start
//!
//! ```
//! use java_lang::ast::CompilationUnit;
//! use java_lang::parse_str;
//!
//! let unit: CompilationUnit = parse_str(
//!     "package com.example;\n\npublic class Hello {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n"
//! ).unwrap();
//!
//! // Access parsed elements
//! if let Some(pkg) = &unit.package {
//!     println!("Package: {:?}", pkg.name);
//! }
//! for item in &unit.type_decls {
//!     println!("Type: {:?}", item);
//! }
//! ```
//!
//! # Design Principles
//!
//! - **Syn-style API**: Uses `Parse` trait and `ParseStream` similar to [`syn`].
//! - **Complete coverage**: Supports Java 25 features including records, sealed classes,
//!   pattern matching, switch expressions, text blocks, and unnamed variables.
//! - **Span tracking**: Every AST node carries span information for error reporting.
//! - **Zero-copy identifiers**: Uses `Ident` with interning for efficient comparison.

mod error;
mod ident;
mod lexer;
mod span;
mod token;
#[macro_use]
mod parse;
mod parser;

pub mod ast;

pub use error::{Error, Result};
pub use ident::Ident;
pub use parse::{Parse, ParseStream, parse, parse_file, parse_str};
pub use span::Span;
pub use token::{Token, TokenKind};

// Re-export the top-level AST types for convenience
pub use ast::compilation_unit::CompilationUnit;
pub use ast::{Comment, CommentKind};
