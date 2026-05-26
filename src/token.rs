use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::span::Span;

/// A token produced by the lexer.
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

/// The kind of token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    IntegerLit(String),
    FloatLit(String),
    BoolLit(bool),
    CharLit(String),
    StringLit(String),
    NullLit,

    // Identifiers and keywords
    Ident(String),
    // Reserved keywords
    Abstract,
    Assert,
    Boolean,
    Break,
    Byte,
    Case,
    Catch,
    Char,
    Class,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extends,
    Final,
    Finally,
    Float,
    For,
    Goto,
    If,
    Implements,
    Import,
    Instanceof,
    Int,
    Interface,
    Long,
    Module,
    Native,
    New,
    Package,
    Private,
    Protected,
    Public,
    Requires,
    Return,
    Short,
    Static,
    Strictfp,
    Super,
    Switch,
    Synchronized,
    This,
    Throw,
    Throws,
    Transient,
    Try,
    Void,
    Volatile,
    While,
    Underscore,
    // Contextual keywords
    Open,
    Opens,
    Exports,
    Provides,
    To,
    Transitive,
    Uses,
    With,
    Yield,
    Var,
    Record,
    Sealed,
    Permits,
    NonSealed,
    When,

    // Separators / punctuation
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Semicolon,  // ;
    Comma,      // ,
    Dot,        // .
    DotDot,     // ..
    Ellipsis,   // ...
    At,         // @
    ColonColon, // ::

    // Operators
    Eq,         // =
    Gt,         // >
    Lt,         // <
    Bang,       // !
    Tilde,      // ~
    Question,   // ?
    Colon,      // :
    Arrow,      // ->
    EqEq,       // ==
    GtEq,       // >=
    LtEq,       // <=
    BangEq,     // !=
    AmpAmp,     // &&
    PipePipe,   // ||
    PlusPlus,   // ++
    MinusMinus, // --
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Amp,        // &
    Pipe,       // |
    Caret,      // ^
    Percent,    // %
    LtLt,       // <<
    GtGt,       // >>
    GtGtGt,     // >>>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    AmpEq,      // &=
    PipeEq,     // |=
    CaretEq,    // ^=
    PercentEq,  // %=
    LtLtEq,     // <<=
    GtGtEq,     // >>=
    GtGtGtEq,   // >>>=

    // End of input
    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerLit(s) => write!(f, "{}", s),
            Self::FloatLit(s) => write!(f, "{}", s),
            Self::BoolLit(b) => write!(f, "{}", b),
            Self::CharLit(s) => write!(f, "'{}'", s),
            Self::StringLit(s) => write!(f, "\"{}\"", s),
            Self::NullLit => write!(f, "null"),
            Self::Ident(s) => write!(f, "{}", s),

            Self::Abstract => write!(f, "abstract"),
            Self::Assert => write!(f, "assert"),
            Self::Boolean => write!(f, "boolean"),
            Self::Break => write!(f, "break"),
            Self::Byte => write!(f, "byte"),
            Self::Case => write!(f, "case"),
            Self::Catch => write!(f, "catch"),
            Self::Char => write!(f, "char"),
            Self::Class => write!(f, "class"),
            Self::Const => write!(f, "const"),
            Self::Continue => write!(f, "continue"),
            Self::Default => write!(f, "default"),
            Self::Do => write!(f, "do"),
            Self::Double => write!(f, "double"),
            Self::Else => write!(f, "else"),
            Self::Enum => write!(f, "enum"),
            Self::Extends => write!(f, "extends"),
            Self::Final => write!(f, "final"),
            Self::Finally => write!(f, "finally"),
            Self::Float => write!(f, "float"),
            Self::For => write!(f, "for"),
            Self::Goto => write!(f, "goto"),
            Self::If => write!(f, "if"),
            Self::Implements => write!(f, "implements"),
            Self::Import => write!(f, "import"),
            Self::Instanceof => write!(f, "instanceof"),
            Self::Int => write!(f, "int"),
            Self::Interface => write!(f, "interface"),
            Self::Long => write!(f, "long"),
            Self::Module => write!(f, "module"),
            Self::Native => write!(f, "native"),
            Self::New => write!(f, "new"),
            Self::Package => write!(f, "package"),
            Self::Private => write!(f, "private"),
            Self::Protected => write!(f, "protected"),
            Self::Public => write!(f, "public"),
            Self::Requires => write!(f, "requires"),
            Self::Return => write!(f, "return"),
            Self::Short => write!(f, "short"),
            Self::Static => write!(f, "static"),
            Self::Strictfp => write!(f, "strictfp"),
            Self::Super => write!(f, "super"),
            Self::Switch => write!(f, "switch"),
            Self::Synchronized => write!(f, "synchronized"),
            Self::This => write!(f, "this"),
            Self::Throw => write!(f, "throw"),
            Self::Throws => write!(f, "throws"),
            Self::Transient => write!(f, "transient"),
            Self::Try => write!(f, "try"),
            Self::Void => write!(f, "void"),
            Self::Volatile => write!(f, "volatile"),
            Self::While => write!(f, "while"),
            Self::Underscore => write!(f, "_"),

            Self::Open => write!(f, "open"),
            Self::Opens => write!(f, "opens"),
            Self::Exports => write!(f, "exports"),
            Self::Provides => write!(f, "provides"),
            Self::To => write!(f, "to"),
            Self::Transitive => write!(f, "transitive"),
            Self::Uses => write!(f, "uses"),
            Self::With => write!(f, "with"),
            Self::Yield => write!(f, "yield"),
            Self::Var => write!(f, "var"),
            Self::Record => write!(f, "record"),
            Self::Sealed => write!(f, "sealed"),
            Self::Permits => write!(f, "permits"),
            Self::NonSealed => write!(f, "non-sealed"),
            Self::When => write!(f, "when"),

            Self::LParen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::LBrace => write!(f, "{{"),
            Self::RBrace => write!(f, "}}"),
            Self::LBracket => write!(f, "["),
            Self::RBracket => write!(f, "]"),
            Self::Semicolon => write!(f, ";"),
            Self::Comma => write!(f, ","),
            Self::Dot => write!(f, "."),
            Self::DotDot => write!(f, ".."),
            Self::Ellipsis => write!(f, "..."),
            Self::At => write!(f, "@"),
            Self::ColonColon => write!(f, "::"),
            Self::Eq => write!(f, "="),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Bang => write!(f, "!"),
            Self::Tilde => write!(f, "~"),
            Self::Question => write!(f, "?"),
            Self::Colon => write!(f, ":"),
            Self::Arrow => write!(f, "->"),
            Self::EqEq => write!(f, "=="),
            Self::GtEq => write!(f, ">="),
            Self::LtEq => write!(f, "<="),
            Self::BangEq => write!(f, "!="),
            Self::AmpAmp => write!(f, "&&"),
            Self::PipePipe => write!(f, "||"),
            Self::PlusPlus => write!(f, "++"),
            Self::MinusMinus => write!(f, "--"),
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Amp => write!(f, "&"),
            Self::Pipe => write!(f, "|"),
            Self::Caret => write!(f, "^"),
            Self::Percent => write!(f, "%"),
            Self::LtLt => write!(f, "<<"),
            Self::GtGt => write!(f, ">>"),
            Self::GtGtGt => write!(f, ">>>"),
            Self::PlusEq => write!(f, "+="),
            Self::MinusEq => write!(f, "-="),
            Self::StarEq => write!(f, "*="),
            Self::SlashEq => write!(f, "/="),
            Self::AmpEq => write!(f, "&="),
            Self::PipeEq => write!(f, "|="),
            Self::CaretEq => write!(f, "^="),
            Self::PercentEq => write!(f, "%="),
            Self::LtLtEq => write!(f, "<<="),
            Self::GtGtEq => write!(f, ">>="),
            Self::GtGtGtEq => write!(f, ">>>="),
            Self::Eof => write!(f, "<eof>"),
        }
    }
}

impl Hash for TokenKind {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Self::Ident(s) => s.hash(state),
            Self::IntegerLit(s) => s.hash(state),
            Self::FloatLit(s) => s.hash(state),
            _ => {}
        }
    }
}

/// Check if a string is a reserved Java keyword.
#[allow(dead_code)]
pub fn is_reserved_keyword(s: &str) -> bool {
    matches!(
        s,
        "abstract"
            | "assert"
            | "boolean"
            | "break"
            | "byte"
            | "case"
            | "catch"
            | "char"
            | "class"
            | "const"
            | "continue"
            | "default"
            | "do"
            | "double"
            | "else"
            | "enum"
            | "extends"
            | "final"
            | "finally"
            | "float"
            | "for"
            | "goto"
            | "if"
            | "implements"
            | "import"
            | "instanceof"
            | "int"
            | "interface"
            | "long"
            | "native"
            | "new"
            | "package"
            | "private"
            | "protected"
            | "public"
            | "requires"
            | "return"
            | "short"
            | "static"
            | "strictfp"
            | "super"
            | "switch"
            | "synchronized"
            | "this"
            | "throw"
            | "throws"
            | "transient"
            | "try"
            | "void"
            | "volatile"
            | "while"
            | "_"
    )
}

/// Check if a string is a contextual keyword.
#[allow(dead_code)]
pub fn is_contextual_keyword(s: &str) -> bool {
    matches!(
        s,
        "open"
            | "opens"
            | "exports"
            | "provides"
            | "to"
            | "transitive"
            | "uses"
            | "with"
            | "yield"
            | "var"
            | "module"
            | "record"
            | "sealed"
            | "permits"
            | "non-sealed"
            | "when"
    )
}

/// Map a keyword string to its token kind. Returns None for non-keywords.
pub fn keyword_to_token(s: &str) -> Option<TokenKind> {
    match s {
        "abstract" => Some(TokenKind::Abstract),
        "assert" => Some(TokenKind::Assert),
        "boolean" => Some(TokenKind::Boolean),
        "break" => Some(TokenKind::Break),
        "byte" => Some(TokenKind::Byte),
        "case" => Some(TokenKind::Case),
        "catch" => Some(TokenKind::Catch),
        "char" => Some(TokenKind::Char),
        "class" => Some(TokenKind::Class),
        "const" => Some(TokenKind::Const),
        "continue" => Some(TokenKind::Continue),
        "default" => Some(TokenKind::Default),
        "do" => Some(TokenKind::Do),
        "double" => Some(TokenKind::Double),
        "else" => Some(TokenKind::Else),
        "enum" => Some(TokenKind::Enum),
        "extends" => Some(TokenKind::Extends),
        "final" => Some(TokenKind::Final),
        "finally" => Some(TokenKind::Finally),
        "float" => Some(TokenKind::Float),
        "for" => Some(TokenKind::For),
        "goto" => Some(TokenKind::Goto),
        "if" => Some(TokenKind::If),
        "implements" => Some(TokenKind::Implements),
        "import" => Some(TokenKind::Import),
        "instanceof" => Some(TokenKind::Instanceof),
        "int" => Some(TokenKind::Int),
        "interface" => Some(TokenKind::Interface),
        "long" => Some(TokenKind::Long),
        "native" => Some(TokenKind::Native),
        "new" => Some(TokenKind::New),
        "package" => Some(TokenKind::Package),
        "private" => Some(TokenKind::Private),
        "protected" => Some(TokenKind::Protected),
        "public" => Some(TokenKind::Public),
        "requires" => Some(TokenKind::Requires),
        "return" => Some(TokenKind::Return),
        "short" => Some(TokenKind::Short),
        "static" => Some(TokenKind::Static),
        "strictfp" => Some(TokenKind::Strictfp),
        "super" => Some(TokenKind::Super),
        "switch" => Some(TokenKind::Switch),
        "synchronized" => Some(TokenKind::Synchronized),
        "this" => Some(TokenKind::This),
        "throw" => Some(TokenKind::Throw),
        "throws" => Some(TokenKind::Throws),
        "transient" => Some(TokenKind::Transient),
        "try" => Some(TokenKind::Try),
        "void" => Some(TokenKind::Void),
        "volatile" => Some(TokenKind::Volatile),
        "while" => Some(TokenKind::While),
        "_" => Some(TokenKind::Underscore),
        // Contextual keywords
        "open" => Some(TokenKind::Open),
        "opens" => Some(TokenKind::Opens),
        "exports" => Some(TokenKind::Exports),
        "provides" => Some(TokenKind::Provides),
        "to" => Some(TokenKind::To),
        "transitive" => Some(TokenKind::Transitive),
        "uses" => Some(TokenKind::Uses),
        "with" => Some(TokenKind::With),
        "yield" => Some(TokenKind::Yield),
        "var" => Some(TokenKind::Var),
        "record" => Some(TokenKind::Record),
        "sealed" => Some(TokenKind::Sealed),
        "permits" => Some(TokenKind::Permits),
        "non-sealed" => Some(TokenKind::NonSealed),
        "when" => Some(TokenKind::When),
        _ => None,
    }
}

/// Check if a token kind is a keyword (reserved or contextual).
#[allow(dead_code)]
pub fn is_keyword(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Abstract
            | TokenKind::Assert
            | TokenKind::Boolean
            | TokenKind::Break
            | TokenKind::Byte
            | TokenKind::Case
            | TokenKind::Catch
            | TokenKind::Char
            | TokenKind::Class
            | TokenKind::Const
            | TokenKind::Continue
            | TokenKind::Default
            | TokenKind::Do
            | TokenKind::Double
            | TokenKind::Else
            | TokenKind::Enum
            | TokenKind::Extends
            | TokenKind::Final
            | TokenKind::Finally
            | TokenKind::Float
            | TokenKind::For
            | TokenKind::Goto
            | TokenKind::If
            | TokenKind::Implements
            | TokenKind::Import
            | TokenKind::Instanceof
            | TokenKind::Int
            | TokenKind::Interface
            | TokenKind::Long
            | TokenKind::Module
            | TokenKind::Native
            | TokenKind::New
            | TokenKind::Package
            | TokenKind::Private
            | TokenKind::Protected
            | TokenKind::Public
            | TokenKind::Requires
            | TokenKind::Return
            | TokenKind::Short
            | TokenKind::Static
            | TokenKind::Strictfp
            | TokenKind::Super
            | TokenKind::Switch
            | TokenKind::Synchronized
            | TokenKind::This
            | TokenKind::Throw
            | TokenKind::Throws
            | TokenKind::Transient
            | TokenKind::Try
            | TokenKind::Void
            | TokenKind::Volatile
            | TokenKind::While
            | TokenKind::Underscore
    )
}

/// Check if a token kind is a primitive type keyword.
pub fn is_primitive_type(kind: &TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Byte
            | TokenKind::Short
            | TokenKind::Int
            | TokenKind::Long
            | TokenKind::Char
            | TokenKind::Float
            | TokenKind::Double
            | TokenKind::Boolean
    )
}

/// Check if a token kind can start a type.
pub fn can_start_type(kind: &TokenKind) -> bool {
    is_primitive_type(kind)
        || matches!(
            kind,
            TokenKind::Ident(_)
                | TokenKind::At // annotation
                | TokenKind::Void
                // contextual keywords that can be types (when used as identifiers in certain contexts)
                | TokenKind::Var
                | TokenKind::Record
                | TokenKind::Sealed
                | TokenKind::Yield
        )
}
