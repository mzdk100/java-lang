mod compilation_unit;
mod import;
mod package;

pub use {compilation_unit::*, import::*, package::*};

use super::{CompilationUnitDeclaration,DocumentationComment};
use crate::{Token, TokenStream};
use nom::{error::ErrorKind, IResult, Input};

fn documentation_comment<'a>(tokens: TokenStream) -> IResult<TokenStream, DocumentationComment<'a>> {
    let (tokens, out) = tokens
        .split_at_position1_complete(|token| !token.is_documentation(), ErrorKind::Complete)?;

    let token = out.iter_elements().next().unwrap();
    let documentation = if let Token::JavaDoc(d) = token {
        d
    } else {
        Default::default()
    };
    Ok((tokens, documentation.into()))
}

fn identifier(tokens: TokenStream) -> IResult<TokenStream, Token> {
    let (tokens, out) =
        tokens.split_at_position1_complete(|token| !token.is_identifier(), ErrorKind::Complete)?;
    let token = out.iter_elements().next().unwrap();
    Ok((tokens, token))
}

pub fn parse<'a>(tokens: TokenStream) -> IResult<TokenStream, CompilationUnitDeclaration<'a>> {
    compilation_unit_declaration(tokens)
}
