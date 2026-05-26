# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A Java 25 AST parser written in Rust, with a syn-style API. Parses Java source into a typed AST following the Java Language Specification (JLS) SE 25.

## Commands

- **Build:** `cargo build`
- **Test all:** `cargo test`
- **Run a single test:** `cargo test <test_name>`
- **Run example:** `cargo run --example hello`
- **Android SDK integration test:** requires `ANDROID_HOME` env var pointing to an Android SDK installation with sources

## Architecture

The crate has three layers: lexing, parsing infrastructure, and AST types.

### Lexing pipeline

`lexer.rs` tokenizes Java source into `Vec<Token>`. Each `Token` holds a `TokenKind` (defined in `token.rs`) and a `Span` (byte offset range from `span.rs`). The lexer handles all Java literals, operators, keywords, and comments. `true`/`false`/`null` are literal tokens, not identifiers.

### Parse trait and ParseStream

The `Parse` trait (`parse.rs`) is modeled after Rust's `syn` crate. Types implement `fn parse(input: &ParseStream) -> Result<Self>` and compose via `input.parse::<T>()`. `ParseStream` wraps a token slice with interior mutability (`Cell<usize>` cursor, `RefCell<Vec<Error>>` errors) so parsing functions take `&ParseStream`, avoiding borrow checker issues in deep recursion.

Key `ParseStream` capabilities:
- `try_parse`, `save_state`/`restore_state` for speculative/backtracking parsing
- `split_gt` for `>>`/`>>>` token splitting in nested generics
- `parse_ident` accepts both real identifiers and contextual keywords
- Combinators: `parse_terminated`, `parse_separated`, `parse_parenthesized`, `parse_braced`, `parse_bracketed`

Top-level entry points: `parse_str`, `parse`, `parse_file`.

### Parser

`parser.rs` (~4000 lines) contains all `Parse` implementations. It is a recursive descent parser with a Pratt parser for expressions (10 precedence levels). Postfix expressions (`.method()`, `[]`, `++`, `::`) are handled in a loop to avoid left-recursion. The cast/lambda/parenthesized-expression ambiguity in `parse_primary_expr` uses lookahead to find `->` after `)`.

### AST types

All AST nodes live in `src/ast/`. The public module structure:

| File | Key types |
|------|-----------|
| `compilation_unit.rs` | `CompilationUnit`, `PackageDecl`, `ImportDecl` |
| `item.rs` | `TypeDecl`, `ClassDecl`, `InterfaceDecl`, `EnumDecl`, `RecordDecl`, `ModuleDecl`, `Modifier`, `ClassBodyDecl`, `MethodDecl`, `ConstructorDecl`, `FormalParameter` |
| `expr.rs` | `Expr` (20 variants), `MethodCallExpr`, `FieldAccessExpr`, `LambdaExpr`, `SwitchExpr`, `NewClassExpr` |
| `stmt.rs` | `Stmt` (19 variants), `Block`, `IfStmt`, `ForStmt`, `TryStmt`, `SwitchStmt` |
| `ty.rs` | `Type`, `PrimitiveType`, `ReferenceType`, `ArrayType` |
| `path.rs` | `Path`, `PathSegment`, `TypeArguments`, `TypeArgument` |
| `lit.rs` | `Lit` (Int, Float, Bool, Char, Str, Null) |
| `op.rs` | `BinOp`, `AssignOp`, `UnaryOp` |
| `pat.rs` | `Pattern`, `TypePattern`, `RecordPattern`, `Guard` |
| `attribute.rs` | `Annotation`, `ElementValuePair`, `ElementValue` |

### Supporting types

- `Ident` (`ident.rs`): identifier with span. Equality/hashing compare only the `name` string, ignoring span. Contextual keywords (`record`, `sealed`, `var`, `yield`, `permits`, `non-sealed`, `when`, etc.) can be used as identifiers in most contexts.
- `Span` (`span.rs`): `(start, end)` byte offsets. Every AST node carries spans. `Span::join` computes composite spans.
- `Error` (`error.rs`): `message` + `span`, displayed as byte offset.

## Key design decisions

- **Non-sealed modifier** is lexed as three tokens (`Ident("non")`, `Minus`, `Sealed`), not one. The modifier parser handles this with lookahead.
- **Numeric literal values are stored as raw strings** (preserving hex, binary, octal, underscores, suffixes) to avoid precision loss.
- **`>>`/`>>>` splitting**: `ParseStream.pending_gts` counter handles nested generics like `List<Map<String, Integer>>`.
- **Speculative parsing**: `try_parse` with state save/restore is used extensively for ambiguous grammar (e.g., cast vs. lambda vs. parenthesized expression).
