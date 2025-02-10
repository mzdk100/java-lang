use std::{
    borrow::Cow,
    fmt::{Display, Formatter,Result as FmtResult}
};
use super::{Annotation, DocumentationComment};

/// PackageDeclaration表示Java程序中的包声明。
/// 它包括包的名称、修饰符和文档注释。
#[derive(Debug, PartialEq)]
pub struct PackageDeclaration<'a> {
    /// 包的名称。
    pub name: Cow<'a, str>,
    /// 应用到包声明的修饰符。
    pub modifiers: Vec<Annotation<'a>>,
    /// 包声明的文档注释。
    pub documentation: Option<DocumentationComment<'a>>,
}

impl<'a> Display for PackageDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
        for i in &self.modifiers {
            Display::fmt(&i, f)?;
        }
        if !self.name.is_empty() {
            write!(f, "package {};\n", self.name)?;
        }

        Ok(())
    }
}

