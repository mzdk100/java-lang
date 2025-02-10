mod compile_unit;
mod import;
mod package;

pub use {compile_unit::*, import::*, package::*};

use super::CompilationUnitDeclaration;
use crate::{Token, TokenStream};
use nom::{error::ErrorKind, IResult, Input};

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

pub fn parse<'a>(tokens: TokenStream) -> IResult<TokenStream, CompilationUnitDeclaration<'a>> {
    compilation_unit_declaration(tokens)
}
