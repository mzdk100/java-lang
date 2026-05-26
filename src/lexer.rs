use std::{cell::RefCell, sync::Arc};

use crate::{
    ident::{is_java_identifier_continue, is_java_identifier_start},
    span::Span,
    token::{Token, TokenKind, keyword_to_token},
};

/// The Java lexer/tokenizer.
pub struct Lexer<'a> {
    input: &'a str,
    chars: RefCell<std::iter::Peekable<std::str::CharIndices<'a>>>,
    offset: RefCell<usize>,
    #[allow(dead_code)]
    source: Arc<str>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: RefCell::new(input.char_indices().peekable()),
            offset: RefCell::new(0),
            source: Arc::from(input),
        }
    }

    #[allow(dead_code)]
    pub fn source(&self) -> Arc<str> {
        self.source.clone()
    }

    /// Tokenize the entire input.
    pub fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace_and_comments();
            if *self.offset.borrow() >= self.input.len() {
                break;
            }
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }
        tokens.push(Token::new(
            TokenKind::Eof,
            Span::new(*self.offset.borrow(), *self.offset.borrow()),
        ));
        tokens
    }

    fn skip_whitespace_and_comments(&self) {
        loop {
            match self.peek_char() {
                None => break,
                Some((_, ch)) if ch.is_whitespace() => {
                    self.advance();
                }
                Some((_, '/')) => {
                    let offset = *self.offset.borrow();
                    let next_offset = offset + '/'.len_utf8();
                    if let Some(ch) = self.input[next_offset..].chars().next() {
                        if ch == '/' {
                            self.skip_line_comment();
                            continue;
                        } else if ch == '*' {
                            self.skip_block_comment();
                            continue;
                        }
                    }
                    break;
                }
                _ => break,
            }
        }
    }

    fn skip_line_comment(&self) {
        self.advance();
        self.advance();
        loop {
            match self.peek_char() {
                None => break,
                Some((_, '\n')) => {
                    self.advance();
                    break;
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
    }

    fn skip_block_comment(&self) {
        self.advance();
        self.advance();
        loop {
            match self.peek_char() {
                None => break,
                Some((_, '*')) => {
                    self.advance();
                    if let Some((_, '/')) = self.peek_char() {
                        self.advance();
                        return;
                    }
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
    }

    fn peek_char(&self) -> Option<(usize, char)> {
        self.chars.borrow_mut().peek().copied()
    }

    fn advance(&self) -> Option<char> {
        let (offset, ch) = self.chars.borrow_mut().next()?;
        *self.offset.borrow_mut() = offset + ch.len_utf8();
        Some(ch)
    }

    fn span_since(&self, start: usize) -> Span {
        Span::new(start, *self.offset.borrow())
    }

    fn next_token(&self) -> Option<Token> {
        let start = *self.offset.borrow();
        let ch = self.advance()?;

        match ch {
            '"' => {
                if self.input[start + 1..].starts_with("\"\"") {
                    *self.offset.borrow_mut() = start + 3;
                    return Some(self.scan_text_block(start));
                }
                Some(self.scan_string(start))
            }
            '\'' => Some(self.scan_char(start)),
            '0'..='9' => self.scan_number(start, ch),
            _ if is_java_identifier_start(ch) => self.scan_identifier_or_keyword(start, ch),
            '(' => Some(Token::new(TokenKind::LParen, self.span_since(start))),
            ')' => Some(Token::new(TokenKind::RParen, self.span_since(start))),
            '{' => Some(Token::new(TokenKind::LBrace, self.span_since(start))),
            '}' => Some(Token::new(TokenKind::RBrace, self.span_since(start))),
            '[' => Some(Token::new(TokenKind::LBracket, self.span_since(start))),
            ']' => Some(Token::new(TokenKind::RBracket, self.span_since(start))),
            ';' => Some(Token::new(TokenKind::Semicolon, self.span_since(start))),
            ',' => Some(Token::new(TokenKind::Comma, self.span_since(start))),
            '.' => self.scan_dot(start),
            '@' => Some(Token::new(TokenKind::At, self.span_since(start))),
            ':' => self.scan_double_or_single(start, ':', TokenKind::ColonColon, TokenKind::Colon),
            '=' => self.scan_double_or_single(start, '=', TokenKind::EqEq, TokenKind::Eq),
            '!' => self.scan_double_or_single(start, '=', TokenKind::BangEq, TokenKind::Bang),
            '~' => Some(Token::new(TokenKind::Tilde, self.span_since(start))),
            '?' => Some(Token::new(TokenKind::Question, self.span_since(start))),
            '+' => self.scan_plus(start),
            '-' => self.scan_minus(start),
            '*' => self.scan_double_or_single(start, '=', TokenKind::StarEq, TokenKind::Star),
            '/' => self.scan_double_or_single(start, '=', TokenKind::SlashEq, TokenKind::Slash),
            '&' => self.scan_amp(start),
            '|' => self.scan_pipe(start),
            '^' => self.scan_double_or_single(start, '=', TokenKind::CaretEq, TokenKind::Caret),
            '%' => self.scan_double_or_single(start, '=', TokenKind::PercentEq, TokenKind::Percent),
            '<' => self.scan_lt(start),
            '>' => self.scan_gt(start),
            _ => Some(Token::new(
                TokenKind::Ident(ch.to_string()),
                self.span_since(start),
            )),
        }
    }

    fn scan_dot(&self, start: usize) -> Option<Token> {
        if let Some((_, next)) = self.peek_char() {
            if next.is_ascii_digit() {
                // Float literal starting with '.': e.g., .5f, .123e10
                let mut num = String::from(".");
                while let Some((_, ch)) = self.peek_char() {
                    match ch {
                        '0'..='9' | '_' => {
                            num.push(ch);
                            self.advance();
                        }
                        'e' | 'E' => {
                            num.push(ch);
                            self.advance();
                            return self.scan_exponent(start, num);
                        }
                        _ => break,
                    }
                }
                return self.finish_number(start, num, true);
            }
            if next == '.' {
                self.advance();
                if let Some((_, '.')) = self.peek_char() {
                    self.advance();
                    return Some(Token::new(TokenKind::Ellipsis, self.span_since(start)));
                }
                return Some(Token::new(TokenKind::DotDot, self.span_since(start)));
            }
        }
        Some(Token::new(TokenKind::Dot, self.span_since(start)))
    }

    fn scan_double_or_single(
        &self,
        start: usize,
        second: char,
        double: TokenKind,
        single: TokenKind,
    ) -> Option<Token> {
        if let Some((_, c)) = self.peek_char()
            && c == second
        {
            self.advance();
            return Some(Token::new(double, self.span_since(start)));
        }
        Some(Token::new(single, self.span_since(start)))
    }

    fn scan_plus(&self, start: usize) -> Option<Token> {
        if let Some((_, '+')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::PlusPlus, self.span_since(start)))
        } else if let Some((_, '=')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::PlusEq, self.span_since(start)))
        } else {
            Some(Token::new(TokenKind::Plus, self.span_since(start)))
        }
    }

    fn scan_minus(&self, start: usize) -> Option<Token> {
        if let Some((_, '-')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::MinusMinus, self.span_since(start)))
        } else if let Some((_, '=')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::MinusEq, self.span_since(start)))
        } else if let Some((_, '>')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::Arrow, self.span_since(start)))
        } else {
            Some(Token::new(TokenKind::Minus, self.span_since(start)))
        }
    }

    fn scan_amp(&self, start: usize) -> Option<Token> {
        if let Some((_, '&')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::AmpAmp, self.span_since(start)))
        } else if let Some((_, '=')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::AmpEq, self.span_since(start)))
        } else {
            Some(Token::new(TokenKind::Amp, self.span_since(start)))
        }
    }

    fn scan_pipe(&self, start: usize) -> Option<Token> {
        if let Some((_, '|')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::PipePipe, self.span_since(start)))
        } else if let Some((_, '=')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::PipeEq, self.span_since(start)))
        } else {
            Some(Token::new(TokenKind::Pipe, self.span_since(start)))
        }
    }

    fn scan_lt(&self, start: usize) -> Option<Token> {
        if let Some((_, '<')) = self.peek_char() {
            self.advance();
            if let Some((_, '=')) = self.peek_char() {
                self.advance();
                Some(Token::new(TokenKind::LtLtEq, self.span_since(start)))
            } else {
                Some(Token::new(TokenKind::LtLt, self.span_since(start)))
            }
        } else if let Some((_, '=')) = self.peek_char() {
            self.advance();
            Some(Token::new(TokenKind::LtEq, self.span_since(start)))
        } else {
            Some(Token::new(TokenKind::Lt, self.span_since(start)))
        }
    }

    fn scan_gt(&self, start: usize) -> Option<Token> {
        match self.peek_char() {
            Some((_, '>')) => {
                self.advance();
                if let Some((_, '>')) = self.peek_char() {
                    self.advance();
                    if let Some((_, '=')) = self.peek_char() {
                        self.advance();
                        Some(Token::new(TokenKind::GtGtEq, self.span_since(start)))
                    } else if let Some((_, '>')) = self.peek_char() {
                        self.advance();
                        if let Some((_, '=')) = self.peek_char() {
                            self.advance();
                            Some(Token::new(TokenKind::GtGtGtEq, self.span_since(start)))
                        } else {
                            Some(Token::new(TokenKind::GtGtGt, self.span_since(start)))
                        }
                    } else {
                        Some(Token::new(TokenKind::GtGt, self.span_since(start)))
                    }
                } else if let Some((_, '=')) = self.peek_char() {
                    self.advance();
                    Some(Token::new(TokenKind::GtGtEq, self.span_since(start)))
                } else {
                    Some(Token::new(TokenKind::GtGt, self.span_since(start)))
                }
            }
            Some((_, '=')) => {
                self.advance();
                Some(Token::new(TokenKind::GtEq, self.span_since(start)))
            }
            _ => Some(Token::new(TokenKind::Gt, self.span_since(start))),
        }
    }

    fn scan_identifier_or_keyword(&self, start: usize, first: char) -> Option<Token> {
        let mut ident = String::new();
        ident.push(first);
        while let Some((_, ch)) = self.peek_char() {
            if is_java_identifier_continue(ch) {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match ident.as_str() {
            "true" => TokenKind::BoolLit(true),
            "false" => TokenKind::BoolLit(false),
            "null" => TokenKind::NullLit,
            s => keyword_to_token(s).unwrap_or(TokenKind::Ident(ident)),
        };
        Some(Token::new(kind, self.span_since(start)))
    }

    fn scan_number(&self, start: usize, first: char) -> Option<Token> {
        let mut num = String::new();
        num.push(first);

        if first == '0'
            && let Some((_, next)) = self.peek_char()
        {
            match next {
                'x' | 'X' => {
                    num.push(next);
                    self.advance();
                    while let Some((_, ch)) = self.peek_char() {
                        match ch {
                            '0'..='9' | 'a'..='f' | 'A'..='F' | '_' => {
                                num.push(ch);
                                self.advance();
                            }
                            '.' => {
                                num.push(ch);
                                self.advance();
                                while let Some((_, ch)) = self.peek_char() {
                                    match ch {
                                        '0'..='9' | 'a'..='f' | 'A'..='F' | '_' => {
                                            num.push(ch);
                                            self.advance();
                                        }
                                        _ => break,
                                    }
                                }
                                if let Some((_, 'p' | 'P')) = self.peek_char() {
                                    num.push(self.advance()?);
                                    if let Some((_, '+' | '-')) = self.peek_char() {
                                        num.push(self.advance()?);
                                    }
                                    while let Some((_, ch)) = self.peek_char() {
                                        if ch.is_ascii_digit() || ch == '_' {
                                            num.push(ch);
                                            self.advance();
                                        } else {
                                            break;
                                        }
                                    }
                                }
                                return self.finish_number(start, num, true);
                            }
                            _ => break,
                        }
                    }
                    return self.finish_number(start, num, false);
                }
                'b' | 'B' => {
                    num.push(next);
                    self.advance();
                    while let Some((_, ch)) = self.peek_char() {
                        match ch {
                            '0' | '1' | '_' => {
                                num.push(ch);
                                self.advance();
                            }
                            _ => break,
                        }
                    }
                    return self.finish_number(start, num, false);
                }
                _ => {
                    while let Some((_, ch)) = self.peek_char() {
                        match ch {
                            '0'..='7' | '_' => {
                                num.push(ch);
                                self.advance();
                            }
                            '.' => {
                                num.push(ch);
                                self.advance();
                                return self.scan_decimal_fraction(start, num);
                            }
                            'e' | 'E' => {
                                num.push(ch);
                                self.advance();
                                return self.scan_exponent(start, num);
                            }
                            _ => break,
                        }
                    }
                    return self.finish_number(start, num, false);
                }
            }
        }

        while let Some((_, ch)) = self.peek_char() {
            match ch {
                '0'..='9' | '_' => {
                    num.push(ch);
                    self.advance();
                }
                '.' => {
                    num.push(ch);
                    self.advance();
                    return self.scan_decimal_fraction(start, num);
                }
                'e' | 'E' => {
                    num.push(ch);
                    self.advance();
                    return self.scan_exponent(start, num);
                }
                _ => break,
            }
        }
        self.finish_number(start, num, false)
    }

    fn scan_decimal_fraction(&self, start: usize, mut num: String) -> Option<Token> {
        while let Some((_, ch)) = self.peek_char() {
            match ch {
                '0'..='9' | '_' => {
                    num.push(ch);
                    self.advance();
                }
                'e' | 'E' => {
                    num.push(ch);
                    self.advance();
                    return self.scan_exponent(start, num);
                }
                _ => break,
            }
        }
        self.finish_number(start, num, true)
    }

    fn scan_exponent(&self, start: usize, mut num: String) -> Option<Token> {
        if let Some((_, '+' | '-')) = self.peek_char() {
            num.push(self.advance()?);
        }
        while let Some((_, ch)) = self.peek_char() {
            if ch.is_ascii_digit() || ch == '_' {
                num.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        self.finish_number(start, num, true)
    }

    fn finish_number(&self, start: usize, mut num: String, is_float: bool) -> Option<Token> {
        let mut is_floating = is_float;
        if let Some((_, 'f' | 'F' | 'd' | 'D')) = self.peek_char() {
            num.push(self.advance()?);
            is_floating = true;
        } else if let Some((_, 'l' | 'L')) = self.peek_char() {
            num.push(self.advance()?);
            is_floating = false;
        }
        let kind = if is_floating {
            TokenKind::FloatLit(num)
        } else {
            TokenKind::IntegerLit(num)
        };
        Some(Token::new(kind, self.span_since(start)))
    }

    fn scan_char_sequence(&self, terminator: char) -> String {
        let mut s = String::new();
        loop {
            match self.peek_char() {
                None => break,
                Some((_, c)) if c == terminator => {
                    self.advance();
                    break;
                }
                Some((_, '\\')) => {
                    self.advance();
                    if let Some(ch) = self.advance() {
                        s.push('\\');
                        s.push(ch);
                    }
                }
                Some((_, ch)) => {
                    s.push(ch);
                    self.advance();
                }
            }
        }
        s
    }

    fn scan_string(&self, start: usize) -> Token {
        let s = self.scan_char_sequence('"');
        Token::new(TokenKind::StringLit(s), self.span_since(start))
    }

    fn scan_text_block(&self, start: usize) -> Token {
        let mut s = String::new();
        // Skip trailing whitespace after opening """
        loop {
            match self.peek_char() {
                None => break,
                Some((_, ' ' | '\t' | '\r')) => {
                    let ch = self.advance().unwrap();
                    s.push(ch);
                }
                Some((_, '\n')) => {
                    self.advance();
                    break;
                }
                _ => break,
            }
        }
        loop {
            match self.peek_char() {
                None => break,
                Some((_, ch)) => {
                    self.advance();
                    if ch == '"' {
                        if let Some((_, '"')) = self.peek_char() {
                            self.advance();
                            if let Some((_, '"')) = self.peek_char() {
                                self.advance();
                                break;
                            } else {
                                s.push('"');
                                s.push('"');
                            }
                        } else {
                            s.push('"');
                        }
                    } else if ch == '\\' {
                        s.push(ch);
                        if let Some(next) = self.advance() {
                            s.push(next);
                        }
                    } else {
                        s.push(ch);
                    }
                }
            }
        }
        Token::new(TokenKind::StringLit(s), self.span_since(start))
    }

    fn scan_char(&self, start: usize) -> Token {
        let s = self.scan_char_sequence('\'');
        Token::new(TokenKind::CharLit(s), self.span_since(start))
    }
}

/// Tokenize a Java source string into a list of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let lexer = Lexer::new(input);
    lexer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let tokens = tokenize("public class Hello { }");
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(kinds[0], TokenKind::Public);
        assert_eq!(kinds[1], TokenKind::Class);
        assert!(matches!(&kinds[2], TokenKind::Ident(s) if s == "Hello"));
        assert_eq!(kinds[3], TokenKind::LBrace);
        assert_eq!(kinds[4], TokenKind::RBrace);
    }

    #[test]
    fn test_operators() {
        let tokens = tokenize("++ -- == != <= >= && || -> :: ...");
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(kinds[0], TokenKind::PlusPlus);
        assert_eq!(kinds[1], TokenKind::MinusMinus);
        assert_eq!(kinds[2], TokenKind::EqEq);
        assert_eq!(kinds[3], TokenKind::BangEq);
        assert_eq!(kinds[4], TokenKind::LtEq);
        assert_eq!(kinds[5], TokenKind::GtEq);
        assert_eq!(kinds[6], TokenKind::AmpAmp);
        assert_eq!(kinds[7], TokenKind::PipePipe);
        assert_eq!(kinds[8], TokenKind::Arrow);
        assert_eq!(kinds[9], TokenKind::ColonColon);
        assert_eq!(kinds[10], TokenKind::Ellipsis);
    }

    #[test]
    fn test_literals() {
        let tokens = tokenize("42 3.14 0xFF 0b1010 true false null \"hello\" 'a'");
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert!(matches!(&kinds[0], TokenKind::IntegerLit(_)));
        assert!(matches!(&kinds[1], TokenKind::FloatLit(_)));
        assert!(matches!(&kinds[2], TokenKind::IntegerLit(_)));
        assert!(matches!(&kinds[3], TokenKind::IntegerLit(_)));
        assert_eq!(kinds[4], TokenKind::BoolLit(true));
        assert_eq!(kinds[5], TokenKind::BoolLit(false));
        assert_eq!(kinds[6], TokenKind::NullLit);
        assert!(matches!(&kinds[7], TokenKind::StringLit(_)));
        assert!(matches!(&kinds[8], TokenKind::CharLit(_)));
    }

    #[test]
    fn test_comments() {
        let tokens = tokenize("a /* comment */ b // line comment\nc");
        let kinds: Vec<_> = tokens.iter().map(|t| t.kind.clone()).collect();
        assert!(matches!(&kinds[0], TokenKind::Ident(s) if s == "a"));
        assert!(matches!(&kinds[1], TokenKind::Ident(s) if s == "b"));
        assert!(matches!(&kinds[2], TokenKind::Ident(s) if s == "c"));
    }

    #[test]
    fn test_text_block() {
        let tokens = tokenize("\"\"\"\nhello\n\"\"\"");
        assert!(matches!(&tokens[0].kind, TokenKind::StringLit(_)));
    }
}
