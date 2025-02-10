use super::{super::PackageDeclaration, identifier, documentation_comment};
use crate::{ts, Token, TokenStream};
use nom::{bytes::tag, combinator::opt, multi::separated_list1, IResult, Parser};
use std::borrow::Cow;

/// 解析包声明从给定的标记流中。
///
/// 这个函数接受一个 `TokenStream` 并返回一个 `PackageDeclaration` 以及剩余的标记。
///
/// # 参数
///
/// * `tokens` - 一个 `TokenStream`，从中解析包声明。
///
/// # 返回
///
/// 包含剩余标记和 `PackageDeclaration` 的元组。
///
/// # 示例
///
/// ```
/// fn main() -> anyhow::Result<()> {
/// use java_lang::{
/// TokenStream,
/// package_declaration
/// };
///
/// let (_, tokens) = TokenStream::from_str(
///     "package com.test;",
/// )?;
/// assert! (!tokens.is_empty());
///
/// let (tokens, package) = package_declaration(tokens)?;
/// assert_eq!(package.name, "com.test");
///
/// assert!(tokens.is_empty());
/// Ok(())
/// }
/// ```
///
/// # 错误
///
/// 如果标记流不包含有效的包声明，这个函数将返回一个错误。
pub fn package_declaration<'a>(
    tokens: TokenStream,
) -> IResult<TokenStream, PackageDeclaration<'a>> {
    let (tokens, documentation) = opt(documentation_comment).parse(tokens)?;
    let (tokens, _) = tag(ts![Package]).parse(tokens)?;
    let (tokens, idents) = separated_list1(tag(ts![Dot]), identifier).parse(tokens)?;
    let (tokens, _) = tag(ts![SemiColon]).parse(tokens)?;
    let name = idents
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(Token::DOT);
    Ok((
        tokens,
        PackageDeclaration {
            name: Cow::Owned(name),
            modifiers: Default::default(),
            documentation,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_declaration() -> anyhow::Result<()> {
        let (_, tokens) = TokenStream::from_str(
            "\
            /** 定义包 */
            package com.test;
            ",
        )?;
        assert!(!tokens.is_empty());

        let (tokens, package) = package_declaration(tokens)?;
        assert_eq!(
            package,
            PackageDeclaration {
                name: "com.test".into(),
                modifiers: Default::default(),
                documentation: Some(" 定义包 ".into())
            }
        );

        assert!(tokens.is_empty());

        Ok(())
    }
}
