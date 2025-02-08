use super::{one_token, Token};
use nom::{
    error::{Error, ErrorKind},
    multi::many0,
    Compare, CompareResult, IResult, Input, Needed, Parser,
};
use std::{borrow::Cow, iter::Enumerate, vec::IntoIter};

#[derive(Clone, Debug)]
pub struct TokenStream {
    data: Cow<'static, Vec<Token>>,
}

impl TokenStream {
    #[inline]
    pub fn from_str(input: &str) -> IResult<&str, Self> {
        let (remaining, out) = many0(one_token).parse(input)?;
        if remaining.trim_end().is_empty() {
            return Ok((
                remaining,
                Self {
                    data: Cow::Owned(out),
                },
            ));
        }
        Err(nom::Err::Failure(Error::new(remaining, ErrorKind::Fail)))
    }

    #[inline]
    pub fn from_vec(data: Vec<Token>) -> Self {
        Self {
            data: Cow::Owned(data),
        }
    }

    #[inline]
    fn from_slice(data: &[Token]) -> Self {
        Self {
            data: Cow::Owned(data.to_owned()),
        }
    }
}

impl Input for TokenStream {
    type Item = Token;
    type Iter = IntoIter<Token>;
    type IterIndices = Enumerate<Self::Iter>;

    #[inline]
    fn input_len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn take(&self, index: usize) -> Self {
        Self::from_slice(&self.data[..index])
    }

    #[inline]
    fn take_from(&self, index: usize) -> Self {
        Self::from_slice(&self.data[index..])
    }

    #[inline]
    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.data.split_at(index);
        (Self::from_slice(suffix), Self::from_slice(prefix))
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        for (i, j) in self.data.iter().enumerate() {
            if predicate(j.clone()) {
                return Some(i);
            }
        }
        None
    }

    #[inline]
    fn iter_elements(&self) -> Self::Iter {
        let data = <Vec<_> as Clone>::clone(&self.data);
        data.into_iter()
    }

    #[inline]
    fn iter_indices(&self) -> Self::IterIndices {
        let data = <Vec<_> as Clone>::clone(&self.data);
        data.into_iter().enumerate()
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        let mut cnt = 0;
        for (index, _) in self.data.iter().enumerate() {
            if cnt == count {
                return Ok(index);
            }
            cnt += 1;
        }
        if cnt == count {
            return Ok(self.input_len());
        }
        Err(Needed::Unknown)
    }
}

impl Compare<TokenStream> for TokenStream {
    #[inline]
    fn compare(&self, t: TokenStream) -> CompareResult {
        let pos = self
            .data
            .iter()
            .zip(t.data.iter())
            .position(|(a, b)| a != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.input_len() >= t.input_len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    #[inline]
    fn compare_no_case(&self, t: TokenStream) -> CompareResult {
        self.compare(t)
    }
}

#[macro_export]
macro_rules! ts {
    ($($token:ident),*) => {
        $crate::TokenStream::from_vec(vec![$($crate::Token::$token),*])
    };
}
