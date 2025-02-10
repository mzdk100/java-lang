use std::{
    borrow::Cow,
    fmt::{Display, Formatter,Result as FmtResult}
};
use crate::{Annotation, DocumentationComment};

/// ModuleDeclaration表示Java程序中的模块声明。
/// 它包括模块的名称、注解、指令以及是否为开放模块。
#[derive(Debug)]
pub struct ModuleDeclaration<'a> {
    /// 模块的名称。
    pub name: Cow<'a, str>,
    /// 应用到模块声明的注解。
    pub annotations: Vec<Annotation<'a>>,
    /// 指定模块的依赖、导出、打开、使用和提供等指令。
    pub directives: Vec<ModuleDirective>,
    /// 是否为开放模块。
    pub open: bool,
    /// 文档注释
    pub documentation: Option<DocumentationComment<'a>>,
}

impl<'a> Display for ModuleDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(ref d) = self.documentation {
            Display::fmt(d, f)?;
        }
        for i in &self.annotations {
            write!(f, "{}\n", i)?;
        }
        if self.open {
            write!(f, "open")?;
        }
        write!(f, "module {} {{}}", self.name)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ModuleDirective;
