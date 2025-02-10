use super::{ClassDeclaration, InterfaceDeclaration};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// TopLevelClassOrInterfaceDeclaration表示Java程序中的顶层类或接口声明。
/// 它可以是类声明或接口声明。
#[derive(Debug)]
pub enum TopLevelClassOrInterfaceDeclaration<'a> {
    /// 表示类声明。
    Class(ClassDeclaration<'a>),
    /// 表示接口声明。
    Interface(InterfaceDeclaration<'a>),
}

impl<'a> Display for TopLevelClassOrInterfaceDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Class(r) => Display::fmt(r, f),
            Self::Interface(r) => Display::fmt(r, f),
        }
    }
}
