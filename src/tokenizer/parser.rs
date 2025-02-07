use super::Token;
use nom::{
    branch::{alt, permutation},
    bytes::complete::{tag, take_until, take_while},
    character::{
        complete::{alpha1, bin_digit1, char, digit1, hex_digit1, oct_digit1},
        one_of,
        streaming::multispace0,
    },
    combinator::{map, opt},
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};

fn keyword(i: &str) -> IResult<&str, Token> {
    let (remaining, out) = alt([
        tag(Token::ABSTRACT),
        tag(Token::ASSERT),
        tag(Token::BOOLEAN),
        tag(Token::BREAK),
        tag(Token::BYTE),
        tag(Token::CASE),
        tag(Token::CATCH),
        tag(Token::CHAR),
        tag(Token::CLASS),
        tag(Token::CONST),
        tag(Token::CONTINUE),
        tag(Token::DEFAULT),
        tag(Token::DO),
        tag(Token::DOUBLE),
        tag(Token::ELSE),
        tag(Token::ENUM),
        tag(Token::EXTENDS),
        tag(Token::FINAL),
        tag(Token::FINALLY),
        tag(Token::FLOAT),
        tag(Token::FOR),
        tag(Token::IF),
        tag(Token::IMPLEMENTS),
        tag(Token::IMPORT),
        tag(Token::INSTANCEOF),
        tag(Token::INT),
        tag(Token::INTERFACE),
        tag(Token::LONG),
        tag(Token::NATIVE),
        tag(Token::NEW),
        tag(Token::PACKAGE),
        tag(Token::PRIVATE),
        tag(Token::PROTECTED),
        tag(Token::PUBLIC),
        tag(Token::RETURN),
        tag(Token::SHORT),
        tag(Token::STATIC),
        tag(Token::STRICTFP),
        tag(Token::SUPER),
        tag(Token::SWITCH),
        tag(Token::SYNCHRONIZED),
        tag(Token::THIS),
        tag(Token::THROW),
        tag(Token::THROWS),
        tag(Token::TRANSIENT),
        tag(Token::TRY),
        tag(Token::VOID),
        tag(Token::VOLATILE),
        tag(Token::WHILE),
    ])
    .parse(i)?;
    Ok((remaining, Token::from(out)))
}

fn identifier(i: &str) -> IResult<&str, Token> {
    let (remaining, out) = alt((alpha1, tag("_"))).parse(i)?;
    let (remaining, out2) =
        take_while(|c: char| c.is_alphanumeric() || c == '_').parse(remaining)?;
    Ok((remaining, Token::Identifier(out.to_string() + out2)))
}

fn boolean_literal(i: &str) -> IResult<&str, Token> {
    map(alt((tag(Token::TRUE), tag(Token::FALSE))), |b: &str| {
        Token::BooleanLiteral(b == Token::TRUE)
    })
    .parse(i)
}

fn null_literal(i: &str) -> IResult<&str, Token> {
    map(tag(Token::NULL), |_| Token::NullLiteral).parse(i)
}

fn number<const RADIX: u8>(i: &str) -> IResult<&str, String> {
    let d = match RADIX {
        2 => bin_digit1,
        8 => oct_digit1,
        10 => digit1,
        16 => hex_digit1,
        _ => unreachable!(),
    };
    map(
        pair(d, many0(preceded(opt(tag("_")), d))),
        |(a, b): (&str, Vec<&str>)| a.to_owned() + b.join("").as_str(),
    )
    .parse(i)
}

fn integer_literal(i: &str) -> IResult<&str, Token> {
    map(number::<10>, |s| Token::IntegerLiteral(s.parse().unwrap())).parse(i)
}

fn double_literal(i: &str) -> IResult<&str, Token> {
    map(
        permutation((number::<10>, tag("."), number::<10>)),
        |(a, _, b)| {
            let a = a.parse().unwrap_or(0f64);
            let l = b.len();
            let b = b.parse().unwrap_or(0f64);
            Token::DoubleLiteral(a + b / 10.0f64.powi(l as _))
        },
    )
    .parse(i)
}

fn float_literal(i: &str) -> IResult<&str, Token> {
    map(
        terminated(
            permutation((number::<10>, tag("."), number::<10>)),
            tag("f"),
        ),
        |(a, _, b)| {
            let a = a.parse().unwrap_or(0f32);
            let l = b.len();
            let b = b.parse().unwrap_or(0f32);
            Token::FloatLiteral(a + b / 10.0f32.powi(l as _))
        },
    )
    .parse(i)
}

fn hex_literal(i: &str) -> IResult<&str, Token> {
    map(
        permutation((tag("0"), one_of("xX"), number::<16>)),
        |(_, _, h)| Token::HexLiteral(u32::from_str_radix(&h, 16).unwrap_or(0)),
    )
    .parse(i)
}

fn oct_literal(i: &str) -> IResult<&str, Token> {
    map(permutation((tag("0"), number::<8>)), |(_, o)| {
        Token::OctLiteral(u32::from_str_radix(&o, 8).unwrap_or(0))
    })
    .parse(i)
}

fn bin_literal(i: &str) -> IResult<&str, Token> {
    map(
        permutation((tag("0"), one_of("bB"), number::<2>)),
        |(_, _, b)| Token::BinLiteral(u32::from_str_radix(&b, 2).unwrap_or(0)),
    )
    .parse(i)
}

fn char_literal(i: &str) -> IResult<&str, Token> {
    map(
        delimited(char('\''), take_while(|c: char| c != '\''), char('\'')),
        |s: &str| Token::CharLiteral(s.chars().next().unwrap()),
    )
    .parse(i)
}

fn string_literal(i: &str) -> IResult<&str, Token> {
    map(
        delimited(char('"'), take_while(|c: char| c != '"'), char('"')),
        |s: &str| Token::StringLiteral(s.to_string()),
    )
    .parse(i)
}

fn separator(i: &str) -> IResult<&str, Token> {
    let (remaining, out) = alt((
        tag(Token::LEFT_PAREN),
        tag(Token::RIGHT_PAREN),
        tag(Token::LEFT_BRACE),
        tag(Token::RIGHT_BRACE),
        tag(Token::LEFT_BRACKET),
        tag(Token::RIGHT_BRACKET),
        tag(Token::SEMI_COLON),
        tag(Token::COMMA),
        tag(Token::DOT),
    ))
    .parse(i)?;
    Ok((remaining, Token::from(out)))
}

fn operator(i: &str) -> IResult<&str, Token> {
    let (remaining, out) = alt([
        tag(Token::UNSIGNED_SHIFT_RIGHT_ASSIGN),
        tag(Token::UNSIGNED_SHIFT_RIGHT),
        tag(Token::SHIFT_RIGHT_ASSIGN),
        tag(Token::SHIFT_LEFT_ASSIGN),
        tag(Token::MOD_ASSIGN),
        tag(Token::XOR_ASSIGN),
        tag(Token::OR_ASSIGN),
        tag(Token::AND_ASSIGN),
        tag(Token::SLASH_ASSIGN),
        tag(Token::STAR_ASSIGN),
        tag(Token::MINUS_ASSIGN),
        tag(Token::PLUS_ASSIGN),
        tag(Token::SHIFT_LEFT),
        tag(Token::SHIFT_RIGHT),
        tag(Token::DOUBLE_MINUS),
        tag(Token::DOUBLE_PLUS),
        tag(Token::LOGICAL_OR),
        tag(Token::LOGICAL_AND),
        tag(Token::NOT_EQUAL),
        tag(Token::GREATER_THAN_OR_EQUAL),
        tag(Token::LESS_THAN_OR_EQUAL),
        tag(Token::DOUBLE_EQUAL),
        tag(Token::MOD),
        tag(Token::XOR),
        tag(Token::OR),
        tag(Token::AND),
        tag(Token::SLASH),
        tag(Token::STAR),
        tag(Token::MINUS),
        tag(Token::PLUS),
        tag(Token::COLON),
        tag(Token::QUESTION),
        tag(Token::NOT),
        tag(Token::LOGICAL_NOT),
        tag(Token::LESS_THAN),
        tag(Token::GREATER_THAN),
        tag(Token::ASSIGN),
        tag(Token::TRIPLE_DOT),
        tag(Token::ARROW),
        tag(Token::DOUBLE_COLON),
    ])
    .parse(i)?;
    Ok((remaining, Token::from(out)))
}

fn single_comment(i: &str) -> IResult<&str, Token> {
    let (remaining, _) = tag("//").parse(i)?;
    let (remaining, out) = take_while(|c| c != '\n' && c != '\r').parse(remaining)?;
    Ok((
        remaining,
        Token::Comment {
            text: out.to_string(),
            single_line: true,
        },
    ))
}

fn multi_comment(i: &str) -> IResult<&str, Token> {
    let (remaining, _) = tag("/*").parse(i)?;
    let (remaining, out) = take_until("*/").parse(remaining)?;
    let (remaining, _) = tag("*/").parse(remaining)?;
    Ok((
        remaining,
        Token::Comment {
            text: out.to_string(),
            single_line: false,
        },
    ))
}

fn java_doc(i: &str) -> IResult<&str, Token> {
    let (remaining, _) = tag("/**").parse(i)?;
    let (remaining, out) = take_until("*/").parse(remaining)?;
    let (remaining, _) = tag("*/").parse(remaining)?;
    Ok((remaining, Token::JavaDoc(out.to_string())))
}

fn one_token(i: &str) -> IResult<&str, Token> {
    let Ok((remaining, _)) = multispace0::<_, Error<_>>(i) else {
        return Err(nom::Err::Error(Error::new("", ErrorKind::Complete)));
    };

    alt((
        java_doc,
        single_comment,
        multi_comment,
        keyword,
        operator,
        boolean_literal,
        null_literal,
        hex_literal,
        oct_literal,
        bin_literal,
        float_literal,
        double_literal,
        integer_literal,
        char_literal,
        string_literal,
        separator,
        identifier,
    ))
    .parse(remaining)
}

pub fn tokenize(i: &str) -> IResult<&str, Vec<Token>> {
    let (remaining, out) = many0(one_token).parse(i)?;
    if remaining.trim_end().is_empty() {
        return Ok((remaining, out));
    }
    Err(nom::Err::Failure(Error::new(remaining, ErrorKind::Fail)))
}
