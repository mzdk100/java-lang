# java-lang

[![Crates.io](https://img.shields.io/crates/v/java-lang)](https://crates.io/crates/java-lang)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#license)

> [English](README.md)

用 Rust 编写的 Java 25 AST 解析器，采用 [syn](https://docs.rs/syn) 风格的 API。将 Java 源代码解析为遵循 Java 语言规范 (JLS) SE 25 的类型化抽象语法树。

## 特性

- **Syn 风格 API** — `Parse` trait 和 `ParseStream`，参照 Rust 的 `syn` crate 设计
- **完整的 Java 25 支持**：
  - Record、密封类、模式匹配（类型模式 + Record 模式）
  - Switch 表达式、文本块
  - Lambda 表达式、方法引用
  - Try-with-resources、`var` 类型推断
  - 模块系统 (JPMS)
  - 注解及所有元素值形式
  - 泛型（通配符和边界）
  - 未命名变量
- **注释解析** — 文档注释（`/** */`、`///`）附加到声明，普通注释（`//`、`/* */`）附加到语句，所有注释可通过 `CompilationUnit` 获取
- **Span 追踪** — 每个 AST 节点携带字节偏移量的 span 信息
- **零拷贝标识符** — `Ident` 仅通过字符串进行相等性比较
- **最小依赖** — 仅依赖 `thiserror` 和 `unicode-xid`

## 快速开始

使用 `cargo add` 安装：

```bash
cargo add java-lang
```

解析 Java 源代码：

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

## 注释

文档注释（`/** ... */` 和 `/// ...`）通过 `doc_comment: Vec<Comment>` 字段附加到其后的声明。普通注释（`// ...` 和 `/* ... */`）通过 `leading_comments: Vec<Comment>` 字段附加到其后的语句。所有注释（包括文档注释和普通注释）也会收集到 `CompilationUnit.comments` 中，便于全文查询。

```rust
use java_lang::ast::CompilationUnit;
use java_lang::parse_str;

let unit: CompilationUnit = parse_str(r#"
    /** 主类 */
    public class Hello {
        // 打印问候语
        public void greet() {
            System.out.println("Hi");
        }
    }
"#).unwrap();

// 类上的文档注释
if let Some(cls) = unit.type_decls.first() {
    // 通过 TypeDecl::Class(cls) 匹配访问 doc_comment
}

// 文件中的所有注释
for comment in &unit.comments {
    println!("注释位置: {:?}", comment.span);
}
```

## AST 遍历

通过匹配 AST 节点变体来遍历解析树：

```rust
use java_lang::ast::{TypeDecl, ClassBodyDecl};

for type_decl in &unit.type_decls {
    match type_decl {
        TypeDecl::Class(cls) => {
            println!("Class: {}", cls.name);
            for member in &cls.body.declarations {
                match member {
                    ClassBodyDecl::Method(m) => {
                        println!("  方法: {} (参数: {})", m.name, m.params.len());
                    }
                    ClassBodyDecl::Field(f) => {
                        for d in &f.declarators {
                            if let Some(name) = &d.name {
                                println!("  字段: {}", name);
                            }
                        }
                    }
                    ClassBodyDecl::Constructor(c) => {
                        println!("  构造函数: {} (参数: {})", c.name, c.params.len());
                    }
                    _ => {}
                }
            }
        }
        TypeDecl::Interface(iface) => {
            println!("接口: {}", iface.name);
        }
        TypeDecl::Enum(e) => {
            println!("枚举: {}", e.name);
        }
        _ => {}
    }
}
```

## API 概览

### 入口函数

| 函数 | 说明 |
|------|------|
| `parse_str<T: Parse>(s: &str) -> Result<T>` | 解析字符串，遇到尾部 token 时报错 |
| `parse<T: Parse>(s: &str) -> Result<T>` | 解析字符串，忽略尾部 token |
| `parse_file<T: Parse>(path: &Path) -> Result<T>` | 从磁盘解析 Java 文件 |

### 核心类型

| 类型 | 说明 |
|------|------|
| `CompilationUnit` | Java 源文件的顶层 AST 节点 |
| `ParseStream` | token 游标，提供组合子方法（`parse_ident`、`parse_braced` 等） |
| `Parse` | trait — 实现 `fn parse(input: &ParseStream) -> Result<Self>` |
| `Ident` | 带 span 的标识符；相等性仅比较名称字符串 |
| `Span` | `(start, end)` 字节偏移量，每个 AST 节点都携带 |
| `Error` | 解析错误，包含消息和 span |

### AST 模块结构

| 模块 | 核心类型 |
|------|----------|
| `ast` | `Comment`、`CommentKind` |
| `ast::compilation_unit` | `CompilationUnit`、`PackageDecl`、`ImportDecl` |
| `ast::item` | `TypeDecl`、`ClassDecl`、`InterfaceDecl`、`EnumDecl`、`RecordDecl`、`MethodDecl`、`ConstructorDecl`、`Modifier` |
| `ast::expr` | `Expr`（20 个变体）、`MethodCallExpr`、`LambdaExpr`、`SwitchExpr` |
| `ast::stmt` | `Stmt`（19 个变体）、`Block`、`IfStmt`、`ForStmt`、`TryStmt`、`SwitchStmt` |
| `ast::ty` | `Type`、`PrimitiveType`、`ReferenceType`、`ArrayType` |
| `ast::path` | `Path`、`PathSegment`、`TypeArguments` |
| `ast::lit` | `Lit` — `Int`、`Float`、`Bool`、`Char`、`Str`、`Null` |
| `ast::op` | `BinOp`、`AssignOp`、`UnaryOp` |
| `ast::pat` | `Pattern`、`TypePattern`、`RecordPattern`、`Guard` |
| `ast::attribute` | `Annotation`、`ElementValuePair`、`ElementValue` |
| `ast::generics` | `TypeParameters`、`TypeParameter`、`TypeBound` |

## 架构

本 crate 分为三层：

```
源代码 → 词法分析器 → Vec<Token> → ParseStream → AST 节点
         (lexer.rs)               (parse.rs)    (ast/*)
```

1. **词法分析** — `lexer.rs` 将 Java 源代码分词为 `Vec<Token>`。每个 token 包含一个 `TokenKind` 和一个 `Span`。注释 token（`LineComment`、`BlockComment`、`DocLineComment`、`DocBlockComment`）也会被输出到 token 流中。
2. **解析基础设施** — `parse.rs` 定义了 `Parse` trait 和 `ParseStream`（内部可变的 token 游标）。通过 `try_parse` 配合状态保存/恢复支持推测性解析。`ParseStream` 在 `peek()`/`advance()` 中透明地跳过注释 token，并将其缓存以供后续通过 `collect_pending_doc_comments()` 和 `collect_pending_comments()` 收集。
3. **AST 类型** — `src/ast/` 包含所有类型化的 AST 节点。解析器（`parser.rs`，约 4000 行）是递归下降解析器，表达式部分采用 Pratt 解析（10 个优先级层次）。

### 设计决策

- **数值字面量存储为原始字符串**（保留十六进制、二进制、八进制、下划线、后缀），避免精度损失。
- **`>>`/`>>>` 拆分** — `ParseStream.pending_gts` 计数器处理嵌套泛型，如 `List<Map<String, Integer>>`。
- **non-sealed 修饰符** — 词法分析为三个 token（`Ident("non")`、`Minus`、`Sealed`），由修饰符解析器通过前向查看处理。
- **上下文关键字**（`record`、`sealed`、`var`、`yield`、`permits`、`non-sealed`、`when`）在大多数上下文中可用作标识符。

## 测试

运行所有测试：

```bash
cargo test
```

运行单个测试：

```bash
cargo test <test_name>
```

运行示例：

```bash
cargo run --example hello
```

Android SDK 集成测试（需要设置 `ANDROID_HOME` 环境变量）：

```bash
cargo test --test android_sdk_tests
```

## 许可证

根据您的选择，许可协议为 [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) 或 [MIT License](http://opensource.org/licenses/MIT)。
