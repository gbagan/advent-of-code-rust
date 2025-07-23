use anyhow::*;
use itertools::traits::HomogeneousTuple;
use std::fmt::Debug;
use std::iter::Enumerate;
use std::marker::PhantomData;
use std::str::pattern::Pattern;
use itertools::Itertools;
use num_integer::Integer;
use num_traits::{ConstZero, Signed};


pub trait Ten {
    const TEN: Self;
}

macro_rules! ten {
    ($($t:ty)*) => ($(
        impl Ten for $t {
            const TEN: $t = 10;
        }
    )*)
}

ten!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);

pub struct ParseUnsigned<'a, T> {
    bytes: std::slice::Iter<'a, u8>,
    phantom: PhantomData<&'a T>,
}

impl<T: Integer + Ten + From<u8>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        next_unsigned(&mut self.bytes)
    }
}

pub struct ParseSigned<'a, T> {
    bytes: std::slice::Iter<'a, u8>,
    phantom: PhantomData<&'a T>,
}

impl<T: Integer + Signed + ConstZero + Ten + From<u8>> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}


fn next_unsigned<T: Integer + Ten + From<u8>>(bytes: &mut std::slice::Iter<'_, u8>) -> Option<T> {
    let mut n = loop {
        let byte = bytes.next()?;
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            break T::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else { return Some(n) };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            return Some(n);
        }
    }
}


fn try_signed<T: Integer + Signed + ConstZero + Ten + From<u8>>(bytes: &mut std::slice::Iter<'_, u8>) -> Option<T> {
    let (mut n, negative) = loop {
        let &byte = bytes.next()?;
        if byte == b'-' {
            break (T::ZERO, true);
        }
        let digit = byte.wrapping_sub(b'0');
        if digit < 10 {
            break (T::from(digit), false);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break Some(if negative { -n } else { n });
        };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = T::TEN * n + T::from(digit);
        } else {
            break Some(if negative { -n } else { n });
        }
    }
}


fn next_lower_case_token<'a>(slice: &'a str, iter: &mut Enumerate<std::str::Bytes<'_>>) -> Option<&'a str> {
    let n = loop {
        let (i, byte) = iter.next()?;
        if byte.is_ascii_lowercase() {
            break i
        }
    };

    for (i, byte) in iter.by_ref() {
        if !byte.is_ascii_lowercase() {
            return Some(&slice[n..i]);
        }
    }
    Some(&slice[n..])
}




pub struct ParseLowercase<'a> {
    slice: &'a str,
    bytes: Enumerate<std::str::Bytes<'a>>,
}

impl<'a> Iterator for ParseLowercase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        next_lower_case_token(self.slice, &mut self.bytes)
    }
}

fn next_upper_case_token<'a>(slice: &'a str, iter: &mut Enumerate<std::str::Bytes<'_>>) -> Option<&'a str> {
    let n = loop {
        let (i, byte) = iter.next()?;
        if byte.is_ascii_uppercase() {
            break i
        }
    };

    for (i, byte) in iter.by_ref() {
        if !byte.is_ascii_uppercase() {
            return Some(&slice[n..i]);
        }
    }
    Some(&slice[n..])
}




pub struct ParseUppercase<'a> {
    slice: &'a str,
    bytes: Enumerate<std::str::Bytes<'a>>,
}

impl<'a> Iterator for ParseUppercase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        next_upper_case_token(self.slice, &mut self.bytes)
    }
}

pub trait ParserIter {
    fn try_unsigned<T: Integer + Ten + From<u8>>(&self) -> Option<T>;
    fn try_signed<T: Integer + Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T>;
    fn iter_unsigned<T: Integer + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Integer + Signed + ConstZero + Ten + From<u8>>(&self) -> ParseSigned<'_, T>;
}

pub trait WordParserIter {
    fn iter_lowercase(&self) -> ParseLowercase<'_>;
    fn iter_uppercase(&self) -> ParseUppercase<'_>;
}

impl ParserIter for &[u8] {
    fn try_signed<T: Integer + Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T> {
        try_signed(&mut self.iter())
    }
    
    fn try_unsigned<T: Integer + Ten + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.iter())
    }


    fn iter_unsigned<T: Integer + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.iter(), phantom: PhantomData }
    }

    fn iter_signed<T: Integer + Signed + Ten + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.iter(), phantom: PhantomData }
    }
}

impl ParserIter for &str {
    fn try_signed<T: Integer + Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T> {
        try_signed(&mut self.as_bytes().iter())
    }
    
    fn try_unsigned<T: Integer + Ten + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.as_bytes().iter())
    }


    fn iter_unsigned<T: Integer + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.as_bytes().iter(), phantom: PhantomData }
    }

    fn iter_signed<T: Integer + Signed + Ten + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.as_bytes().iter(), phantom: PhantomData }
    }
}

impl WordParserIter for &str {
    fn iter_lowercase(&self) -> ParseLowercase<'_> {
        ParseLowercase { slice: self, bytes: self.bytes().enumerate() }
    }

    fn iter_uppercase(&self) -> ParseUppercase<'_> {
        ParseUppercase { slice: self, bytes: self.bytes().enumerate() }
    }
}

pub trait TryParseLines {
    fn try_parse_lines_and_collect<A, C, F>(self, f: F) -> Result<C>
    where
        Self: Sized,
        F: Fn(Self) -> Result<A>,
        Result<C>: FromIterator<Result<A>>;

    fn try_split_once<P>(self, delim: P) -> Result<(Self, Self)>
    where
        Self: Sized,
        P: Pattern + Debug + Copy;

    fn try_split_into_tuple<P, T>(self, delim: P) -> Result<T>
    where 
        Self: Sized,
        P: Pattern + Debug + Copy,
        T: HomogeneousTuple<Item=Self>;

    /* 
    fn try_rsplit_once<'a, P>(self, delim: P) -> Result<(Self, Self)>
    where
        Self: Sized,
        P: Pattern + Debug + Copy,
        <P as Pattern>::Searcher<'a>: ReverseSearcher<'a>;
    */

    fn try_unsigned_tuple<U, T>(self) -> Result<T>
    where 
        Self: Sized,
        T: HomogeneousTuple<Item=U>,
        U: Integer + Ten + From<u8>;
}

impl TryParseLines for &str {
    #[inline]
    fn try_parse_lines_and_collect<A, C, F>(self, f: F) -> Result<C>
    where
        Self: Sized,
        F: Fn(Self) -> Result<A>,
        Result<C>: FromIterator<Result<A>>
    {
        self
            .lines()
            .map(|line| f(line).with_context(|| format!("Parse error on line: '{line}'")))
            .try_collect()
    }

    #[inline]
    fn try_split_once<'a, P>(self, delimiter: P) -> Result<(Self, Self)>
        where
            Self: Sized,
            P: Pattern + Debug + Copy
    {
        self.split_once(delimiter).with_context(|| format!("No delimiter '{delimiter:?}' found in string '{self}'"))
    }
    /* 
    fn try_rsplit_once<'a, P>(self, delimiter: P) -> Result<(Self, Self)>
        where
            Self: Sized,
            P: Pattern + Debug + Copy,
            <P as Pattern>::Searcher<'a>: ReverseSearcher<'a>,
    {
        self.rsplit_once(delimiter).with_context(|| format!("No delimiter '{delimiter:?}' found in string '{self}'"))
    } */

    #[inline]
    fn try_split_into_tuple<P, T>(self, delimiter: P) -> Result<T>
    where 
        Self: Sized,
        P: Pattern + Debug + Copy,
        T: HomogeneousTuple<Item=Self>
    {
        self.split(delimiter)
            .collect_tuple()
            .with_context(|| format!("'{delimiter:?}' must split the string in exactly {} tokens'", T::num_items()))
    }

    #[inline]
    fn try_unsigned_tuple<U, T>(self) -> Result<T>
    where 
        Self: Sized,
        T: HomogeneousTuple<Item=U>,
        U: Integer + Ten + From<u8>
    {
        self.iter_unsigned()
            .collect_tuple()
            .with_context(|| format!("String must contains exaactly {} integers", T::num_items()))
    }


}   

#[test]

fn test_iter_lowercase() {
    let input = "abcXb?cdAa";
    let output: Vec<_> = input.iter_lowercase().collect();
    assert_eq!(output, vec!("abc", "b", "cd", "a"));
}