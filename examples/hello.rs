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
//! let java_source = r#"
//! package com.example;
//!
//! import java.util.List;
//!
//! public class Hello {
//!     private String name;
//!
//!     public Hello(String name) {
//!         this.name = name;
//!     }
//!
//!     public void greet() {
//!         System.out.println("Hello, " + name + "!");
//!     }
//!
//!     public static void main(String[] args) {
//!         var hello = new Hello("World");
//!         hello.greet();
//!     }
//! }
//! "#;
//!
//! let unit: CompilationUnit = parse_str(java_source).unwrap();
//!
//! // Access parsed elements
//! if let Some(pkg) = &unit.package {
//!     println!("Package: {}", pkg.name);
//! }
//!
//! for import_decl in &unit.imports {
//!     println!("Import: {:?}", import_decl);
//! }
//!
//! for type_decl in &unit.type_decls {
//!     match type_decl {
//!         java_lang::ast::TypeDecl::Class(cls) => {
//!             println!("Class: {}", cls.name);
//!             for member in &cls.body.declarations {
//!                 match member {
//!                     java_lang::ast::ClassBodyDecl::Method(m) => {
//!                         println!("  Method: {}", m.name);
//!                     }
//!                     java_lang::ast::ClassBodyDecl::Field(f) => {
//!                         for decl in &f.declarators {
//!                             if let Some(name) = &decl.name {
//!                                 println!("  Field: {}", name);
//!                             }
//!                         }
//!                     }
//!                     _ => {}
//!                 }
//!             }
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//!
//! # Features
//!
//! - **Syn-style API**: Uses `Parse` trait and `ParseStream` similar to [`syn`].
//! - **Complete coverage**: Supports Java 25 features including:
//!   - Records, sealed classes, pattern matching
//!   - Switch expressions, text blocks
//!   - Lambda expressions, method references
//!   - Try-with-resources, var type inference
//!   - Module system (JPMS)
//!   - Annotations with all forms
//!   - Generics with wildcards and bounds
//! - **Span tracking**: Every AST node carries span information for error reporting.
//! - **Zero-copy identifiers**: Uses `Ident` with efficient string comparison.

fn main() {
    let source = r#"
package com.example;

import java.util.List;
import java.util.Map;

/**
 * A simple example class demonstrating the parser.
 */
public class Example {
    private final String name;
    private int count;

    public Example(String name, int count) {
        this.name = name;
        this.count = count;
    }

    public String getName() {
        return this.name;
    }

    public int getCount() {
        return this.count;
    }

    public void increment() {
        this.count++;
    }

    @Override
    public String toString() {
        return "Example{name='" + this.name + "', count=" + this.count + "}";
    }

    public static void main(String[] args) {
        var example = new Example("test", 42);
        System.out.println(example);
    }
}
"#;

    let unit: java_lang::ast::CompilationUnit = java_lang::parse_str(source).unwrap();

    println!("=== Parsed Java Source ===\n");

    if let Some(pkg) = &unit.package {
        println!("Package: {}", pkg.name);
    }

    println!("\nImports ({}):", unit.imports.len());
    for imp in &unit.imports {
        println!("  - {:?}", imp);
    }

    println!("\nType declarations ({}):", unit.type_decls.len());
    for td in &unit.type_decls {
        match td {
            java_lang::ast::TypeDecl::Class(cls) => {
                println!("  class {} ({:?})", cls.name, cls.name.span);
                for modifier in &cls.modifiers {
                    println!("    modifier: {:?}", modifier);
                }
                for member in &cls.body.declarations {
                    match member {
                        java_lang::ast::ClassBodyDecl::Field(f) => {
                            for d in &f.declarators {
                                let name = d.name.as_ref().map(|n| n.name.as_str()).unwrap_or("_");
                                println!("    field: {} (type: {:?})", name, f.ty);
                            }
                        }
                        java_lang::ast::ClassBodyDecl::Method(m) => {
                            println!("    method: {} (params: {})", m.name, m.params.len());
                        }
                        java_lang::ast::ClassBodyDecl::Constructor(c) => {
                            println!("    constructor: {} (params: {})", c.name, c.params.len());
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
