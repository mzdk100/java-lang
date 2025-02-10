use super::{super::ImportDeclaration, identifier};
use crate::{ts, Token, TokenStream};
use nom::{
    bytes::tag,
    combinator::{complete, opt},
    error::Error,
    multi::{many0, separated_list1},
    IResult, Parser,
};
use std::borrow::Cow;

/// 解析导入声明
///
/// 该函数解析一个导入声明，并返回解析后的导入声明和剩余的标记流。
///
/// # 参数
///
/// * `tokens` - 标记流，包含待解析的标记。
///
/// # 返回值
///
/// 返回一个 `IResult`，其中包含解析后剩余的标记流和导入声明。
///
/// # 示例
///
/// ```rust
/// use java_lang::{TokenStream,import_declaration};
/// let (_, tokens) = TokenStream::from_str("import java.util.List;")?;
/// let (_, import_declaration) = import_declaration(tokens)?;
/// ```
///
/// # 错误处理
///
/// 如果解析过程中出现错误，将返回一个解析错误。
pub fn import_declaration<'a>(tokens: TokenStream) -> IResult<TokenStream, ImportDeclaration<'a>> {
    let (tokens, _) = tag(ts![Import]).parse(tokens)?;
    let (tokens, r#static) = opt(tag(ts![Static])).parse(tokens)?;
    let (tokens, idents) = separated_list1(complete(tag(ts![Dot])), identifier).parse(tokens)?;
    let name = idents
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(Token::DOT);

    let Ok((tokens, _)) = tag::<_, _, Error<TokenStream>>(ts![Dot, Star]).parse(tokens.clone())
    else {
        let (tokens, _) = tag(ts![SemiColon]).parse(tokens)?;
        let import_declaration = if r#static.is_none() {
            ImportDeclaration::SimpleType(Cow::Owned(name))
        } else {
            ImportDeclaration::SingleStatic(Cow::Owned(name))
        };
        return Ok((tokens, import_declaration));
    };

    let (tokens, _) = tag(ts![SemiColon]).parse(tokens)?;
    let import_declaration = if r#static.is_none() {
        ImportDeclaration::TypeOnDemand(Cow::Owned(name))
    } else {
        ImportDeclaration::StaticOnDemand(Cow::Owned(name))
    };
    Ok((tokens, import_declaration))
}

/// 解析导入声明列表
///
/// 该函数解析一个导入声明列表，并返回解析后的导入声明列表和剩余的标记流。
///
/// # 参数
///
/// * `tokens` - 标记流，包含待解析的标记。
///
/// # 返回值
///
/// 返回一个 `IResult`，其中包含解析后剩余的标记流和导入声明列表。
///
/// # 示例
///
/// ```rust
/// use java_lang::{TokenStream,import_declarations};
/// let (_, tokens) = TokenStream::from_str("import java.util.List;")?;
/// let (_, import_declarations) = import_declarations(tokens)?;
/// ```
///
/// # 错误处理
///
/// 如果解析过程中出现错误，将返回一个解析错误。
pub fn import_declarations<'a>(
    tokens: TokenStream,
) -> IResult<TokenStream, Vec<ImportDeclaration<'a>>> {
    many0(complete(import_declaration)).parse(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMPORTS: &str = "
    import java.util.List;
    import java.io.*;
    import static java.util.Collections.emptyList;
    import static java.util.Collections.*;
    ";

    #[test]
    fn test_import_declaration() -> anyhow::Result<()> {
        let (_, tokens) = TokenStream::from_str(IMPORTS)?;
        assert!(!tokens.is_empty());

        let (tokens, import) = import_declaration(tokens)?;
        assert!(matches!(import, ImportDeclaration::SimpleType(_)));

        let (tokens, import) = import_declaration(tokens)?;
        assert!(matches!(import, ImportDeclaration::TypeOnDemand(_)));

        let (tokens, import) = import_declaration(tokens)?;
        assert!(matches!(import, ImportDeclaration::SingleStatic(_)));

        let (tokens, import) = import_declaration(tokens)?;
        assert!(matches!(import, ImportDeclaration::StaticOnDemand(_)));

        assert!(tokens.is_empty());

        Ok(())
    }

    #[test]
    fn test_import_declarations() -> anyhow::Result<()> {
        let (_, tokens) = TokenStream::from_str(IMPORTS)?;
        assert!(!tokens.is_empty());

        let (tokens, imports) = import_declarations(tokens)?;
        assert_eq!(imports.len(), 4);

        assert!(tokens.is_empty());

        Ok(())
    }
}
