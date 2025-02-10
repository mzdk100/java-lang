use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// ImportDeclaration 枚举表示Java中的导入声明。
#[derive(Debug)]
pub enum ImportDeclaration<'a> {
    /// 单类型导入声明，参数是导入的类或接口的名称。
    SimpleType(Cow<'a, str>),
    /// 需求类型导入声明，参数是导入的包、类或接口的名称（路径）。
    TypeOnDemand(Cow<'a, str>),
    /// 单静态导入声明，参数是导入的类或接口的名称 + 导入的静态成员的名称。
    SingleStatic(Cow<'a, str>),
    /// 静态需求导入声明，参数是导入的类或接口的名称。
    StaticOnDemand(Cow<'a, str>),
}

impl<'a> Display for ImportDeclaration<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::SimpleType(r) | Self::SingleStatic(r) => write!(f, "import {};", r),
            Self::TypeOnDemand(r) | Self::StaticOnDemand(r) => write!(f, "import {}.*;", r),
        }
    }
}
