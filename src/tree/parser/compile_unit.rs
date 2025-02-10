use super::{
    super::{import_declarations, package_declaration},
    CompilationUnitDeclaration,
};
use crate::TokenStream;
use nom::{combinator::opt, IResult, Parser};

/// 解析编译单元声明从给定的标记流中。
///
/// 这个函数接受一个 `TokenStream` 并返回一个 `CompilationUnitDeclaration` 以及剩余的标记。
///
/// # 参数
///
/// * `tokens` - 一个 `TokenStream`，从中解析编译单元声明。
///
/// # 返回
///
/// 包含剩余标记和 `CompilationUnitDeclaration` 的元组。
///
/// # 示例
///
/// ```
/// use java_lang::{
/// TokenStream,
/// compilation_unit_declaration
/// };
///
/// let (_, tokens) = TokenStream::from_str("
/// package com.test;
/// import java.io.File;
/// ")?;
/// assert!(!tokens.is_empty());
///
/// let (tokens, cu) = compilation_unit_declaration(tokens)?;
/// assert_eq!(cu.package.name, "com.test");
/// assert_eq!(cu.imports.len(), 1);
///
/// assert!(tokens.is_empty());
/// ```
///
/// # 错误
///
/// 如果标记流不包含有效的编译单元声明，这个函数将返回一个错误。
pub fn compilation_unit_declaration<'a>(
    tokens: TokenStream,
) -> IResult<TokenStream, CompilationUnitDeclaration<'a>> {
    let (tokens, package) = opt(package_declaration).parse(tokens)?;
    let (tokens, imports) = import_declarations(tokens)?;

    Ok((
        tokens,
        CompilationUnitDeclaration::new(package.unwrap_or_default(), imports, Default::default()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_unit() -> anyhow::Result<()> {
        let (_, tokens) = TokenStream::from_str(
            "\
            package com.test;\
            import java.io.File;
            ",
        )?;
        assert!(!tokens.is_empty());

        let (tokens, cu) = compilation_unit_declaration(tokens)?;
        assert_eq!(cu.package.name, "com.test");
        assert_eq!(cu.imports.len(), 1);

        assert!(tokens.is_empty());

        Ok(())
    }
}
