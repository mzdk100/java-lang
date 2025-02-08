use super::{CompilationUnit, PackageDeclaration};
use crate::{ts, Token, TokenStream};
use nom::{
    bytes::tag, combinator::opt, error::ErrorKind, multi::separated_list1, IResult, Input, Parser,
};
use std::borrow::Cow;

fn java_doc(tokens: TokenStream) -> IResult<TokenStream, Token> {
    let (tokens, out) = tokens
        .split_at_position1_complete(|token| !token.is_documentation(), ErrorKind::Complete)?;

    let token = out.iter_elements().next().unwrap();
    Ok((tokens, token))
}

fn identifier(tokens: TokenStream) -> IResult<TokenStream, Token> {
    let (tokens, out) =
        tokens.split_at_position1_complete(|token| !token.is_identifier(), ErrorKind::Complete)?;
    let token = out.iter_elements().next().unwrap();
    Ok((tokens, token))
}

fn compilation_unit<'a>(tokens: TokenStream) -> IResult<TokenStream, CompilationUnit<'a>> {
    let (tokens, package) = package_declaration(tokens)?;
    Ok((
        tokens,
        CompilationUnit::new(package, Default::default(), Default::default()),
    ))
}

fn package_declaration<'a>(tokens: TokenStream) -> IResult<TokenStream, PackageDeclaration<'a>> {
    let (tokens, doc) = opt(java_doc).parse(tokens)?;
    let (tokens, _) = tag(ts![Package]).parse(tokens)?;
    let (tokens, idents) = separated_list1(tag(ts![Dot]), identifier).parse(tokens)?;
    let (tokens, _) = tag(ts![SemiColon]).parse(tokens)?;
    let name = idents
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(Token::DOT);
    let doc = doc
        .map(|i| match i {
            Token::JavaDoc(s) => s,
            _ => Default::default(),
        })
        .unwrap_or_default();
    Ok((
        tokens,
        PackageDeclaration::new(Cow::Owned(name), Default::default(), Cow::Owned(doc)),
    ))
}

pub fn parse<'a>(tokens: TokenStream) -> IResult<TokenStream, CompilationUnit<'a>> {
    compilation_unit(tokens)
}
