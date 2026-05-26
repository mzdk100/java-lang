# java-lang

[![Crates.io](https://img.shields.io/crates/v/java-lang)](https://crates.io/crates/java-lang)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)

> [中文文档](README.zh.md)

A Java 25 AST parser written in Rust, with a [syn](https://docs.rs/syn)-style API. Parses Java source code into a typed Abstract Syntax Tree following the Java Language Specification (JLS) SE 25.

## Features

- **Syn-style API** — `Parse` trait and `ParseStream` modeled after Rust's `syn` crate
- **Complete Java 25 coverage**:
  - Records, sealed classes, pattern matching (type + record patterns)
  - Switch expressions, text blocks
  - Lambda expressions, method references
  - Try-with-resources, `var` type inference
  - Module system (JPMS)
  - Annotations with all element value forms
  - Generics with wildcards and bounds
  - Unnamed variables
- **Comment parsing** — doc comments (`/** */`, `///`) attached to declarations, regular comments (`//`, `/* */`) attached to statements, all comments available on `CompilationUnit`
- **Span tracking** — every AST node carries byte-offset span information
- **Zero-copy identifiers** — `Ident` with efficient string-only equality
- **Minimal dependencies** — only `thiserror` and `unicode-xid`

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
java-lang = "0.1.0"
```

Parse a Java source file:

```rust
use java_lang::ast::CompilationUnit;
use java_lang::parse_str;

let unit: CompilationUnit = parse_str(
    "package com.example;\n\npublic class Hello {\n    public static void main(String[] args) {\n        System.out.println(\"Hello, World!\");\n    }\n}\n"
).unwrap();

if let Some(pkg) = &unit.package {
    println!("Package: {:?}", pkg.name);
}
for item in &unit.type_decls {
    println!("Type: {:?}", item);
}
```

## Comments

Doc comments (`/** ... */` and `/// ...`) are attached to the declaration they precede via a `doc_comment: Vec<Comment>` field. Regular comments (`// ...` and `/* ... */`) are attached to the statement they precede via `leading_comments: Vec<Comment>`. All comments (both doc and regular) are also collected in `CompilationUnit.comments` for full-source querying.

```rust
use java_lang::ast::CompilationUnit;
use java_lang::parse_str;

let unit: CompilationUnit = parse_str(r#"
    /** Main class */
    public class Hello {
        // Print a greeting
        public void greet() {
            System.out.println("Hi");
        }
    }
"#).unwrap();

// Doc comment on the class
if let Some(cls) = unit.type_decls.first() {
    // Access doc_comment via match on TypeDecl::Class(cls)
}

// All comments in the file
for comment in &unit.comments {
    println!("Comment at {:?}", comment.span);
}
```

## AST Traversal

Walk the parsed tree by matching on AST node variants:

```rust
use java_lang::ast::{TypeDecl, ClassBodyDecl};

for type_decl in &unit.type_decls {
    match type_decl {
        TypeDecl::Class(cls) => {
            println!("Class: {}", cls.name);
            for member in &cls.body.declarations {
                match member {
                    ClassBodyDecl::Method(m) => {
                        println!("  Method: {} (params: {})", m.name, m.params.len());
                    }
                    ClassBodyDecl::Field(f) => {
                        for d in &f.declarators {
                            if let Some(name) = &d.name {
                                println!("  Field: {}", name);
                            }
                        }
                    }
                    ClassBodyDecl::Constructor(c) => {
                        println!("  Constructor: {} (params: {})", c.name, c.params.len());
                    }
                    _ => {}
                }
            }
        }
        TypeDecl::Interface(iface) => {
            println!("Interface: {}", iface.name);
        }
        TypeDecl::Enum(e) => {
            println!("Enum: {}", e.name);
        }
        _ => {}
    }
}
```

## API Overview

### Entry Points

| Function | Description |
|----------|-------------|
| `parse_str<T: Parse>(s: &str) -> Result<T>` | Parse a string, error on trailing tokens |
| `parse<T: Parse>(s: &str) -> Result<T>` | Parse a string, ignore trailing tokens |
| `parse_file<T: Parse>(path: &Path) -> Result<T>` | Parse a Java file from disk |

### Key Types

| Type | Description |
|------|-------------|
| `CompilationUnit` | Top-level AST node for a Java source file |
| `ParseStream` | Token cursor with combinators (`parse_ident`, `parse_braced`, etc.) |
| `Parse` | Trait — implement `fn parse(input: &ParseStream) -> Result<Self>` |
| `Ident` | Identifier with span; equality compares only the name string |
| `Span` | `(start, end)` byte offsets carried by every AST node |
| `Error` | Parse error with message and span |

### AST Module Structure

| Module | Key Types |
|--------|-----------|
| `ast` | `Comment`, `CommentKind` |
| `ast::compilation_unit` | `CompilationUnit`, `PackageDecl`, `ImportDecl` |
| `ast::item` | `TypeDecl`, `ClassDecl`, `InterfaceDecl`, `EnumDecl`, `RecordDecl`, `MethodDecl`, `ConstructorDecl`, `Modifier` |
| `ast::expr` | `Expr` (20 variants), `MethodCallExpr`, `LambdaExpr`, `SwitchExpr` |
| `ast::stmt` | `Stmt` (19 variants), `Block`, `IfStmt`, `ForStmt`, `TryStmt`, `SwitchStmt` |
| `ast::ty` | `Type`, `PrimitiveType`, `ReferenceType`, `ArrayType` |
| `ast::path` | `Path`, `PathSegment`, `TypeArguments` |
| `ast::lit` | `Lit` — `Int`, `Float`, `Bool`, `Char`, `Str`, `Null` |
| `ast::op` | `BinOp`, `AssignOp`, `UnaryOp` |
| `ast::pat` | `Pattern`, `TypePattern`, `RecordPattern`, `Guard` |
| `ast::attribute` | `Annotation`, `ElementValuePair`, `ElementValue` |
| `ast::generics` | `TypeParameters`, `TypeParameter`, `TypeBound` |

## Architecture

The crate has three layers:

```
Source text → Lexer → Vec<Token> → ParseStream → AST nodes
              (lexer.rs)          (parse.rs)     (ast/*)
```

1. **Lexing** — `lexer.rs` tokenizes Java source into `Vec<Token>`. Each token holds a `TokenKind` and a `Span`. Comment tokens (`LineComment`, `BlockComment`, `DocLineComment`, `DocBlockComment`) are emitted into the token stream.
2. **Parsing infrastructure** — `parse.rs` defines the `Parse` trait and `ParseStream` (interior-mutable token cursor). Supports speculative parsing via `try_parse` with save/restore state. `ParseStream` transparently skips comment tokens in `peek()`/`advance()`, buffering them for later collection via `collect_pending_doc_comments()` and `collect_pending_comments()`.
3. **AST types** — `src/ast/` contains all typed AST nodes. The parser (`parser.rs`, ~4000 lines) is a recursive descent parser with a Pratt parser for expressions (10 precedence levels).

### Design Decisions

- **Numeric literals are stored as raw strings** (preserving hex, binary, octal, underscores, suffixes) to avoid precision loss.
- **`>>`/`>>>` splitting** — `ParseStream.pending_gts` counter handles nested generics like `List<Map<String, Integer>>`.
- **Non-sealed modifier** — lexed as three tokens (`Ident("non")`, `Minus`, `Sealed`), handled by the modifier parser with lookahead.
- **Contextual keywords** (`record`, `sealed`, `var`, `yield`, `permits`, `non-sealed`, `when`) can be used as identifiers in most contexts.

## Testing

Run all tests:

```bash
cargo test
```

Run a single test:

```bash
cargo test <test_name>
```

Run the example:

```bash
cargo run --example hello
```

Android SDK integration test (requires `ANDROID_HOME` env var):

```bash
cargo test --test android_sdk_tests
```

## License

Licensed under either of [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [MIT License](http://opensource.org/licenses/MIT) at your option.
