use std::fmt::{Display, Formatter, Result as FmtResult};
use super::{ImportDeclaration, ModuleDeclaration, PackageDeclaration, TopLevelClassOrInterfaceDeclaration};

/// CompilationUnitDeclaration表示一个编译单元，它是Java程序语法语法的终极符号。
/// 它可以是普通编译单元或模块编译单元。
#[derive(Debug)]
pub enum CompilationUnitDeclaration<'a> {
    /// 表示一个普通编译单元。
    /// 它由可选的包声明、import声明和顶层类或接口声明组成。
    Ordinary {
        /// 可选的包声明，指定编译单元所属的包的完全限定名。
        package: Option<PackageDeclaration<'a>>,
        /// Import声明，允许使用简单名称引用其他包中的类和接口。
        imports: Vec<ImportDeclaration<'a>>,
        /// 类和接口的顶层声明。
        top_level_class_or_interfaces: Vec<TopLevelClassOrInterfaceDeclaration<'a>>,
    },
    /// 表示一个模块编译单元。
    /// 它由import声明和模块声明组成。
    Modular {
        /// Import声明，允许在模块声明中引用此模块和其它模块中的包中的类和接口。
        imports: Vec<ImportDeclaration<'a>>,
        /// 模块声明，指定编译单元所属的模块。
        module: ModuleDeclaration<'a>,
    },
}

impl<'a> CompilationUnitDeclaration<'a> {
    /// 获取包声明
    pub fn package(&self) -> Option<&PackageDeclaration> {
        if let Self::Ordinary {
            package: Some(package),
            ..
        } = self
        {
            return Some(package);
        }
        None
    }

    /// 获取导入声明
    pub fn imports(&self) -> &[ImportDeclaration] {
        match self {
            Self::Ordinary { imports, .. } | Self::Modular { imports, .. } => imports,
        }
    }
}

impl<'a> Display for CompilationUnitDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(package) = self.package() {
            Display::fmt(package, f)?;
        }
        for i in self.imports() {
            Display::fmt(&i, f)?;
            write!(f, "\n")?;
        }
        if let Self::Ordinary {
            top_level_class_or_interfaces,
            ..
        } = self
        {
            for i in top_level_class_or_interfaces {
                Display::fmt(&i, f)?;
                write!(f, "\n")?;
            }
        }
        if let Self::Modular { module, .. } = self {
            Display::fmt(module, f)?;
        }

        Ok(())
    }
}

