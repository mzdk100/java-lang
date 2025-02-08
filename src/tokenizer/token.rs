use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// abstract
    Abstract,
    /// assert
    Assert,
    /// boolean
    Boolean,
    /// break
    Break,
    /// byte
    Byte,
    /// case
    Case,
    /// catch
    Catch,
    /// char
    Char,
    /// class
    Class,
    /// const
    Const,
    /// continue
    Continue,
    /// default
    Default,
    /// do
    Do,
    /// double
    Double,
    /// else
    Else,
    /// enum
    Enum,
    /// extends
    Extends,
    /// final
    Final,
    /// finally
    Finally,
    /// float
    Float,
    /// for
    For,
    /// if
    If,
    /// implements
    Implements,
    /// import
    Import,
    /// instanceof
    Instanceof,
    /// int
    Int,
    /// interface
    Interface,
    /// long
    Long,
    /// native
    Native,
    /// new
    New,
    /// package
    Package,
    /// private
    Private,
    /// protected
    Protected,
    /// public
    Public,
    /// return
    Return,
    /// short
    Short,
    /// static
    Static,
    /// strictfp
    Strictfp,
    /// super
    Super,
    /// switch
    Switch,
    /// synchronized
    Synchronized,
    /// this
    This,
    /// throw
    Throw,
    /// throws
    Throws,
    /// transient
    Transient,
    /// try
    Try,
    /// void
    Void,
    /// volatile
    Volatile,
    /// while
    While,

    /// 注释
    Comment { text: String, single_line: bool },

    /// 任意有效标识符
    Identifier(String),
    /// 任意布尔值
    BooleanLiteral(bool),
    /// 任意字符字面量
    CharLiteral(char),
    /// 任意有效整数字面量
    IntegerLiteral(i32),
    /// 任意双精度小数字面量
    DoubleLiteral(f64),
    /// 任意浮点数字面量
    FloatLiteral(f32),
    /// 任意十六进制数字
    HexLiteral(u32),
    /// 任意八进制数字
    OctLiteral(u32),
    /// 任意二进制数字
    BinLiteral(u32),
    /// 任意字符串字面量
    StringLiteral(String),
    /// 空值字面量
    NullLiteral,

    /// 右移 `>>`
    ShiftRight,
    /// 无符号右移 `>>>`
    UnsignedShiftRight,

    /// 分号 `;`
    SemiColon,
    /// 逗号 `,`
    Comma,
    /// 点 `.` (用于访问成员)
    Dot,
    /// 左括号 `(` (用于方法调用或类型转换)
    LeftParen,
    /// 右括号 `)` (用于方法调用或类型转换)
    RightParen,
    /// 左大括号 `{` (用于代码块或对象初始化)
    LeftBrace,
    /// 右大括号 `}` (用于代码块或对象初始化)
    RightBrace,
    /// 左方括号 `[` (用于数组访问)
    LeftBracket,
    /// 右方括号 `]` (用于数组访问)
    RightBracket,

    /// Java文档注释
    JavaDoc(String),

    /// 无符号右移赋值 `>>>=`
    UnsignedShiftRightAssign,
    /// 右移赋值 `>>=`
    ShiftRightAssign,
    /// 左移赋值 `<<=`
    ShiftLeftAssign,
    /// 取模赋值 `%=`
    ModAssign,
    /// 位异或赋值 `^=`
    XorAssign,
    /// 位或赋值 `|=`
    OrAssign,
    /// 位与赋值 `&=`
    AndAssign,
    /// 除法赋值 `/=`
    SlashAssign,

    /// 乘法赋值 `*=`
    StarAssign,
    /// 减法赋值 `-=`
    MinusAssign,
    /// 加法赋值 `+=`
    PlusAssign,
    /// 左移 `<<`
    ShiftLeft,
    /// 双减 `--` (用于自减)
    DoubleMinus,
    /// 双加 `++` (用于自增)
    DoublePlus,
    /// 逻辑或 `||`
    LogicalOr,
    /// 逻辑与 `&&`
    LogicalAnd,
    /// 不等于 `!=`
    NotEqual,

    /// 大于等于 `>=`
    GreaterThanOrEqual,
    /// 小于等于 `<=`
    LessThanOrEqual,
    /// 双等号 `==` (用于比较)
    DoubleEqual,
    /// 取模 `%`
    Mod,
    /// 位异或 `^`
    Xor,
    /// 位或 `|`
    Or,
    /// 位与 `&`
    And,
    /// 除号 `/`
    Slash,
    /// 乘号 `*`
    Star,
    /// 减号 `-`
    Minus,

    /// 加号 `+`
    Plus,
    /// 冒号 `:`
    Colon,
    /// 三元运算符 `?`
    Question,
    /// 位非 `~`
    Not,
    /// 逻辑非 `!`
    LogicalNot,
    /// 小于 `<`
    LessThan,
    /// 大于 `>`
    GreaterThan,
    /// 赋值 `=`
    Assign,
    /// 三点 `...` (用于可变参数或数组切片)
    TripleDot,
    /// 箭头 `->` (用于Lambda表达式)
    Arrow,
    /// 双冒号 `::` (用于方法引用或包访问)
    DoubleColon,
}

impl Token {
    pub(crate) const ABSTRACT: &'static str = "abstract";
    pub(crate) const ASSERT: &'static str = "assert";
    pub(crate) const BOOLEAN: &'static str = "boolean";
    pub(crate) const BREAK: &'static str = "break";
    pub(crate) const BYTE: &'static str = "byte";
    pub(crate) const CASE: &'static str = "case";
    pub(crate) const CATCH: &'static str = "catch";
    pub(crate) const CHAR: &'static str = "char";
    pub(crate) const CLASS: &'static str = "class";
    pub(crate) const CONST: &'static str = "const";
    pub(crate) const CONTINUE: &'static str = "continue";
    pub(crate) const DEFAULT: &'static str = "default";
    pub(crate) const DO: &'static str = "do";
    pub(crate) const DOUBLE: &'static str = "double";
    pub(crate) const ELSE: &'static str = "else";
    pub(crate) const ENUM: &'static str = "enum";
    pub(crate) const EXTENDS: &'static str = "extends";
    pub(crate) const FINAL: &'static str = "final";
    pub(crate) const FINALLY: &'static str = "finally";
    pub(crate) const FLOAT: &'static str = "float";
    pub(crate) const FOR: &'static str = "for";
    pub(crate) const IF: &'static str = "if";
    pub(crate) const IMPLEMENTS: &'static str = "implements";
    pub(crate) const IMPORT: &'static str = "import";
    pub(crate) const INSTANCEOF: &'static str = "instanceof";
    pub(crate) const INT: &'static str = "int";
    pub(crate) const INTERFACE: &'static str = "interface";
    pub(crate) const LONG: &'static str = "long";
    pub(crate) const NATIVE: &'static str = "native";
    pub(crate) const NEW: &'static str = "new";
    pub(crate) const PACKAGE: &'static str = "package";
    pub(crate) const PRIVATE: &'static str = "private";
    pub(crate) const PROTECTED: &'static str = "protected";
    pub(crate) const PUBLIC: &'static str = "public";
    pub(crate) const RETURN: &'static str = "return";
    pub(crate) const SHORT: &'static str = "short";
    pub(crate) const STATIC: &'static str = "static";
    pub(crate) const STRICTFP: &'static str = "strictfp";
    pub(crate) const SUPER: &'static str = "super";
    pub(crate) const SWITCH: &'static str = "switch";
    pub(crate) const SYNCHRONIZED: &'static str = "synchronized";
    pub(crate) const THIS: &'static str = "this";
    pub(crate) const THROW: &'static str = "throw";
    pub(crate) const THROWS: &'static str = "throws";
    pub(crate) const TRANSIENT: &'static str = "transient";
    pub(crate) const TRY: &'static str = "try";
    pub(crate) const VOID: &'static str = "void";
    pub(crate) const VOLATILE: &'static str = "volatile";
    pub(crate) const WHILE: &'static str = "while";

    pub(crate) const NULL: &'static str = "null";

    pub(crate) const TRUE: &'static str = "true";
    pub(crate) const FALSE: &'static str = "false";

    pub(crate) const SEMI_COLON: &'static str = ";";
    pub(crate) const COMMA: &'static str = ",";
    pub(crate) const DOT: &'static str = ".";
    pub(crate) const LEFT_PAREN: &'static str = "(";
    pub(crate) const RIGHT_PAREN: &'static str = ")";
    pub(crate) const LEFT_BRACE: &'static str = "{";
    pub(crate) const RIGHT_BRACE: &'static str = "}";
    pub(crate) const LEFT_BRACKET: &'static str = "[";
    pub(crate) const RIGHT_BRACKET: &'static str = "]";

    pub(crate) const UNSIGNED_SHIFT_RIGHT_ASSIGN: &'static str = ">>>=";
    pub(crate) const UNSIGNED_SHIFT_RIGHT: &'static str = ">>>";
    pub(crate) const SHIFT_RIGHT_ASSIGN: &'static str = ">>=";
    pub(crate) const SHIFT_LEFT_ASSIGN: &'static str = "<<=";
    pub(crate) const MOD_ASSIGN: &'static str = "%=";
    pub(crate) const XOR_ASSIGN: &'static str = "^=";
    pub(crate) const OR_ASSIGN: &'static str = "|=";
    pub(crate) const AND_ASSIGN: &'static str = "&=";
    pub(crate) const SLASH_ASSIGN: &'static str = "/=";

    pub(crate) const STAR_ASSIGN: &'static str = "*=";
    pub(crate) const MINUS_ASSIGN: &'static str = "-=";
    pub(crate) const PLUS_ASSIGN: &'static str = "+=";
    pub(crate) const SHIFT_LEFT: &'static str = "<<";
    pub(crate) const SHIFT_RIGHT: &'static str = ">>";

    pub(crate) const DOUBLE_MINUS: &'static str = "--";
    pub(crate) const DOUBLE_PLUS: &'static str = "++";
    pub(crate) const LOGICAL_OR: &'static str = "||";
    pub(crate) const LOGICAL_AND: &'static str = "&&";
    pub(crate) const NOT_EQUAL: &'static str = "!=";

    pub(crate) const GREATER_THAN_OR_EQUAL: &'static str = ">=";
    pub(crate) const LESS_THAN_OR_EQUAL: &'static str = "<=";
    pub(crate) const DOUBLE_EQUAL: &'static str = "==";
    pub(crate) const MOD: &'static str = "%";
    pub(crate) const XOR: &'static str = "^";
    pub(crate) const OR: &'static str = "|";
    pub(crate) const AND: &'static str = "&";
    pub(crate) const SLASH: &'static str = "/";
    pub(crate) const STAR: &'static str = "*";
    pub(crate) const MINUS: &'static str = "-";

    pub(crate) const COLON: &'static str = ":";
    pub(crate) const QUESTION: &'static str = "?";
    pub(crate) const NOT: &'static str = "~";
    pub(crate) const LOGICAL_NOT: &'static str = "!";
    pub(crate) const LESS_THAN: &'static str = "<";
    pub(crate) const GREATER_THAN: &'static str = ">";
    pub(crate) const TRIPLE_DOT: &'static str = "...";
    pub(crate) const ARROW: &'static str = "->";
    pub(crate) const DOUBLE_COLON: &'static str = "::";
    pub(crate) const ASSIGN: &'static str = "=";
    pub(crate) const PLUS: &'static str = "+";

    pub fn is_method_reference(&self) -> bool {
        &Self::DoubleColon == self
    }

    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::Assert
                | Self::Boolean
                | Self::Break
                | Self::Byte
                | Self::Case
                | Self::Catch
                | Self::Char
                | Self::Class
                | Self::Const
                | Self::Continue
                | Self::Default
                | Self::Do
                | Self::Double
                | Self::Else
                | Self::Enum
                | Self::Extends
                | Self::Final
                | Self::Finally
                | Self::Float
                | Self::For
                | Self::If
                | Self::Implements
                | Self::Import
                | Self::Instanceof
                | Self::Int
                | Self::Interface
                | Self::Long
                | Self::Native
                | Self::New
                | Self::Package
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Return
                | Self::Short
                | Self::Static
                | Self::Strictfp
                | Self::Super
                | Self::Switch
                | Self::Synchronized
                | Self::This
                | Self::Throw
                | Self::Throws
                | Self::Transient
                | Self::Try
                | Self::Void
                | Self::Volatile
                | Self::While
        )
    }

    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            Self::Abstract
                | Self::Default
                | Self::Final
                | Self::Native
                | Self::Private
                | Self::Protected
                | Self::Public
                | Self::Static
                | Self::Strictfp
                | Self::Synchronized
                | Self::Transient
                | Self::Volatile
        )
    }

    pub fn is_basic_type(&self) -> bool {
        matches!(
            self,
            Self::Boolean
                | Self::Byte
                | Self::Char
                | Self::Double
                | Self::Float
                | Self::Int
                | Self::Long
                | Self::Short
        )
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Self::BooleanLiteral(_)
                | Self::IntegerLiteral(_)
                | Self::BinLiteral(_)
                | Self::FloatLiteral(_)
                | Self::HexLiteral(_)
                | Self::OctLiteral(_)
                | Self::CharLiteral(_)
                | Self::StringLiteral(_)
                | Self::NullLiteral
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::IntegerLiteral(_))
    }

    pub fn is_octal(&self) -> bool {
        matches!(self, Self::OctLiteral(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::BinLiteral(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::DoubleLiteral(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::FloatLiteral(_))
    }

    pub fn is_hex(&self) -> bool {
        matches!(self, Self::HexLiteral(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::BooleanLiteral(_))
    }

    pub fn is_character(&self) -> bool {
        matches!(self, Self::CharLiteral(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::StringLiteral(_))
    }

    pub fn is_null(&self) -> bool {
        &Self::NullLiteral == self // 需要自定义解析逻辑
    }

    pub fn is_documentation(&self) -> bool {
        matches!(self, Self::JavaDoc(_))
    }

    pub fn is_separator(&self) -> bool {
        matches!(
            self,
            Self::SemiColon
                | Self::Comma
                | Self::Dot
                | Self::LeftParen
                | Self::RightParen
                | Self::LeftBrace
                | Self::RightBrace
                | Self::LeftBracket
                | Self::RightBracket
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            // '>>>=',5a3 '>>=', '<<=',  '%=', '^=', '|=', '&=', '/=',
            Self::UnsignedShiftRightAssign
                | Self::ShiftRightAssign
                | Self::ShiftLeftAssign
                | Self::ModAssign
                | Self::XorAssign
                | Self::OrAssign
                | Self::AndAssign
                | Self::SlashAssign

            // '*=', '-=', '+=', '<<', '--', '++', '||', '&&', '!=',
                | Self::StarAssign
                | Self::MinusAssign
                | Self::PlusAssign
                | Self::ShiftLeft
                | Self::DoubleMinus
                | Self::DoublePlus
                | Self::LogicalOr
                | Self::LogicalAnd
                | Self::NotEqual

            // '>=', '<=', '==', '%', '^', '|', '&', '/', '*', '-',
                | Self::GreaterThanOrEqual
                | Self::LessThanOrEqual
                | Self::DoubleEqual
                | Self::Mod
                | Self::Xor
                | Self::Or
                | Self::And
                | Self::Slash
                | Self::Star
                | Self::Minus

            // '+', ':', '?', '~', '!', '<', '>', '=', '...', '->', '::'
                | Self::Plus
                | Self::Colon
                | Self::Question
                | Self::Not
                | Self::LogicalNot
                | Self::LessThan
                | Self::GreaterThan
                | Self::Assign
                | Self::TripleDot
                | Self::Arrow
                | Self::DoubleColon
        )
    }

    pub fn is_annotation(&self) -> bool {
        false // 需要自定义解析逻辑
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            Self::ABSTRACT => Self::Abstract,
            Self::ASSERT => Self::Assert,
            Self::BOOLEAN => Self::Boolean,
            Self::BREAK => Self::Break,
            Self::BYTE => Self::Byte,
            Self::CASE => Self::Case,
            Self::CATCH => Self::Catch,
            Self::CHAR => Self::Char,
            Self::CLASS => Self::Class,
            Self::CONST => Self::Const,
            Self::CONTINUE => Self::Continue,
            Self::DEFAULT => Self::Default,
            Self::DO => Self::Do,
            Self::DOUBLE => Self::Double,
            Self::ELSE => Self::Else,
            Self::ENUM => Self::Enum,
            Self::EXTENDS => Self::Extends,
            Self::FINAL => Self::Final,
            Self::FINALLY => Self::Finally,
            Self::FLOAT => Self::Float,
            Self::FOR => Self::For,
            Self::IF => Self::If,
            Self::IMPLEMENTS => Self::Implements,
            Self::IMPORT => Self::Import,
            Self::INSTANCEOF => Self::Instanceof,
            Self::INT => Self::Int,
            Self::INTERFACE => Self::Interface,
            Self::LONG => Self::Long,
            Self::NATIVE => Self::Native,
            Self::NEW => Self::New,
            Self::PACKAGE => Self::Package,
            Self::PRIVATE => Self::Private,
            Self::PROTECTED => Self::Protected,
            Self::PUBLIC => Self::Public,
            Self::RETURN => Self::Return,
            Self::SHORT => Self::Short,
            Self::STATIC => Self::Static,
            Self::STRICTFP => Self::Strictfp,
            Self::SUPER => Self::Super,
            Self::SWITCH => Self::Switch,
            Self::SYNCHRONIZED => Self::Synchronized,
            Self::THIS => Self::This,
            Self::THROW => Self::Throw,
            Self::THROWS => Self::Throws,
            Self::TRANSIENT => Self::Transient,
            Self::TRY => Self::Try,
            Self::VOID => Self::Void,
            Self::VOLATILE => Self::Volatile,
            Self::WHILE => Self::While,
            Self::NULL => Self::NullLiteral,
            Self::SEMI_COLON => Self::SemiColon,
            Self::COMMA => Self::Comma,
            Self::DOT => Self::Dot,
            Self::LEFT_BRACE => Self::LeftBrace,
            Self::RIGHT_BRACE => Self::RightBrace,
            Self::LEFT_BRACKET => Self::LeftBracket,
            Self::RIGHT_BRACKET => Self::RightBracket,
            Self::LEFT_PAREN => Self::LeftParen,
            Self::RIGHT_PAREN => Self::RightParen,
            Self::UNSIGNED_SHIFT_RIGHT_ASSIGN => Self::UnsignedShiftRightAssign,
            Self::UNSIGNED_SHIFT_RIGHT => Self::UnsignedShiftRight,
            Self::SHIFT_RIGHT_ASSIGN => Self::ShiftRightAssign,
            Self::SHIFT_LEFT_ASSIGN => Self::ShiftLeftAssign,
            Self::MOD_ASSIGN => Self::ModAssign,
            Self::XOR_ASSIGN => Self::XorAssign,
            Self::OR_ASSIGN => Self::OrAssign,
            Self::AND_ASSIGN => Self::AndAssign,
            Self::SLASH_ASSIGN => Self::SlashAssign,
            Self::STAR_ASSIGN => Self::StarAssign,
            Self::MINUS_ASSIGN => Self::MinusAssign,
            Self::PLUS_ASSIGN => Self::PlusAssign,
            Self::SHIFT_LEFT => Self::ShiftLeft,
            Self::SHIFT_RIGHT => Self::ShiftRight,
            Self::DOUBLE_MINUS => Self::DoubleMinus,
            Self::DOUBLE_PLUS => Self::DoublePlus,
            Self::LOGICAL_OR => Self::LogicalOr,
            Self::LOGICAL_AND => Self::LogicalAnd,
            Self::NOT_EQUAL => Self::NotEqual,
            Self::GREATER_THAN_OR_EQUAL => Self::GreaterThanOrEqual,
            Self::LESS_THAN_OR_EQUAL => Self::LessThanOrEqual,
            Self::DOUBLE_EQUAL => Self::DoubleEqual,
            Self::MOD => Self::Mod,
            Self::XOR => Self::Xor,
            Self::OR => Self::Or,
            Self::AND => Self::And,
            Self::SLASH => Self::Slash,
            Self::STAR => Self::Star,
            Self::MINUS => Self::Minus,
            Self::PLUS => Self::Plus,
            Self::COLON => Self::Colon,
            Self::QUESTION => Self::Question,
            Self::NOT => Self::Not,
            Self::LOGICAL_NOT => Self::LogicalNot,
            Self::LESS_THAN => Self::LessThan,
            Self::GREATER_THAN => Self::GreaterThan,
            Self::ASSIGN => Self::Assign,
            Self::TRIPLE_DOT => Self::TripleDot,
            Self::ARROW => Self::Arrow,
            Self::DOUBLE_COLON => Self::DoubleColon,
            _ => Self::Identifier(value.to_string()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Abstract => Self::ABSTRACT,
            Self::Assert => Self::ASSERT,
            Self::Boolean => Self::BOOLEAN,
            Self::Break => Self::BREAK,
            Self::Byte => Self::BYTE,
            Self::Case => Self::CASE,
            Self::Catch => Self::CATCH,
            Self::Char => Self::CHAR,
            Self::Class => Self::CLASS,
            Self::Const => Self::CONST,
            Self::Continue => Self::CONTINUE,
            Self::Default => Self::DEFAULT,
            Self::Do => Self::DO,
            Self::Double => Self::DOUBLE,
            Self::Else => Self::ELSE,
            Self::Enum => Self::ENUM,
            Self::Extends => Self::EXTENDS,
            Self::Final => Self::FINAL,
            Self::Finally => Self::FINALLY,
            Self::Float => Self::FLOAT,
            Self::For => Self::FOR,
            Self::If => Self::IF,
            Self::Implements => Self::IMPLEMENTS,
            Self::Import => Self::IMPORT,
            Self::Instanceof => Self::INSTANCEOF,
            Self::Int => Self::INT,
            Self::Interface => Self::INTERFACE,
            Self::Long => Self::LONG,
            Self::Native => Self::NATIVE,
            Self::New => Self::NEW,
            Self::Package => Self::PACKAGE,
            Self::Private => Self::PRIVATE,
            Self::Protected => Self::PROTECTED,
            Self::Public => Self::PUBLIC,
            Self::Return => Self::RETURN,
            Self::Short => Self::SHORT,
            Self::Static => Self::STATIC,
            Self::Strictfp => Self::STRICTFP,
            Self::Super => Self::SUPER,
            Self::Switch => Self::SWITCH,
            Self::Synchronized => Self::SYNCHRONIZED,
            Self::This => Self::THIS,
            Self::Throw => Self::THROW,
            Self::Throws => Self::THROWS,
            Self::Transient => Self::TRANSIENT,
            Self::Try => Self::TRY,
            Self::Void => Self::VOID,
            Self::Volatile => Self::VOLATILE,
            Self::While => Self::WHILE,
            Self::Comment {
                text,
                single_line: true,
            } => return write!(f, "//{}", text),
            Self::Comment {
                text,
                single_line: false,
            } => return write!(f, "/*{}*/", text),
            Self::Identifier(i) => i.as_str(),
            Self::CharLiteral(c) => return write!(f, "'{}'", c),
            Self::BooleanLiteral(b) => return write!(f, "{}", b),
            Self::IntegerLiteral(i) => return write!(f, "{}", i),
            Self::DoubleLiteral(i) => return write!(f, "{}", i),
            Self::FloatLiteral(i) => return write!(f, "{}f", i),
            Self::HexLiteral(h) => return write!(f, "0x{:x}", h),
            Self::OctLiteral(o) => return write!(f, "0{:o}", o),
            Self::BinLiteral(b) => return write!(f, "0b{:b}", b),
            Self::StringLiteral(s) => return write!(f, "\"{}\"", s),
            Self::NullLiteral => Self::NULL,
            Self::ShiftRight => Self::SHIFT_RIGHT,
            Self::UnsignedShiftRight => Self::UNSIGNED_SHIFT_RIGHT,
            Self::SemiColon => Self::SEMI_COLON,
            Self::Comma => Self::COMMA,
            Self::Dot => Self::DOT,
            Self::LeftParen => Self::LEFT_PAREN,
            Self::RightParen => Self::RIGHT_PAREN,
            Self::LeftBrace => Self::LEFT_BRACE,
            Self::RightBrace => Self::RIGHT_BRACE,
            Self::LeftBracket => Self::LEFT_BRACKET,
            Self::RightBracket => Self::RIGHT_BRACKET,
            Self::JavaDoc(s) => return write!(f, "/**{}*/", s),
            Self::UnsignedShiftRightAssign => Self::UNSIGNED_SHIFT_RIGHT_ASSIGN,
            Self::ShiftRightAssign => Self::SHIFT_RIGHT_ASSIGN,
            Self::ShiftLeftAssign => Self::SHIFT_LEFT_ASSIGN,
            Self::ModAssign => Self::MOD_ASSIGN,
            Self::XorAssign => Self::XOR_ASSIGN,
            Self::OrAssign => Self::OR_ASSIGN,
            Self::AndAssign => Self::AND_ASSIGN,
            Self::SlashAssign => Self::SLASH_ASSIGN,
            Self::StarAssign => Self::STAR_ASSIGN,
            Self::MinusAssign => Self::MINUS_ASSIGN,
            Self::PlusAssign => Self::PLUS_ASSIGN,
            Self::ShiftLeft => Self::SHIFT_LEFT,
            Self::DoubleMinus => Self::DOUBLE_MINUS,
            Self::DoublePlus => Self::DOUBLE_PLUS,
            Self::LogicalOr => Self::LOGICAL_OR,
            Self::LogicalAnd => Self::LOGICAL_AND,
            Self::NotEqual => Self::NOT_EQUAL,
            Self::GreaterThanOrEqual => Self::GREATER_THAN_OR_EQUAL,
            Self::LessThanOrEqual => Self::LESS_THAN_OR_EQUAL,
            Self::DoubleEqual => Self::DOUBLE_EQUAL,
            Self::Mod => Self::MOD,
            Self::Xor => Self::XOR,
            Self::Or => Self::OR,
            Self::And => Self::AND,
            Self::Slash => Self::SLASH,
            Self::Star => Self::STAR,
            Self::Minus => Self::MINUS,
            Self::Plus => Self::PLUS,
            Self::Colon => Self::COLON,
            Self::Question => Self::QUESTION,
            Self::Not => Self::NOT,
            Self::LogicalNot => Self::LOGICAL_NOT,
            Self::LessThan => Self::LESS_THAN,
            Self::GreaterThan => Self::GREATER_THAN,
            Self::Assign => Self::ASSIGN,
            Self::TripleDot => Self::TRIPLE_DOT,
            Self::Arrow => Self::ARROW,
            Self::DoubleColon => Self::DOUBLE_COLON,
        };
        write!(f, "{}", text)
    }
}
