use std::iter::Enumerate;
use std::marker::PhantomData;
use num_traits::{ConstZero, Num, Signed};


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

impl<T: Num + Ten + From<u8>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        next_unsigned(&mut self.bytes)
    }
}

pub struct ParseSigned<'a, T> {
    bytes: std::slice::Iter<'a, u8>,
    phantom: PhantomData<&'a T>,
}

impl<T: Signed + ConstZero + Ten + From<u8>> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        try_signed(&mut self.bytes)
    }
}

fn to_unsigned<T: Num + Ten + From<u8>>(bytes: &[u8]) -> T {
    bytes.iter().fold(T::zero(), |acc, &n| T::TEN * acc + T::from(n - b'0'))
}

fn next_unsigned<T: Num + Ten + From<u8>>(bytes: &mut std::slice::Iter<'_, u8>) -> Option<T> {
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


fn try_signed<T: Signed + ConstZero + Ten + From<u8>>(bytes: &mut std::slice::Iter<'_, u8>) -> Option<T> {
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

fn next_upper_case_token<'a>(slice: &'a [u8], iter: &mut Enumerate<std::str::Bytes<'_>>) -> Option<&'a [u8]> {
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
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        next_upper_case_token(self.slice.as_bytes(), &mut self.bytes)
    }
}

pub trait ParserIter {
    fn to_unsigned<T: Num + Ten + From<u8>>(&self) -> T;
    fn try_unsigned<T: Num + Ten + From<u8>>(&self) -> Option<T>;
    fn try_signed<T: Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T>;
    fn iter_unsigned<T: Num + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Signed + ConstZero + Ten + From<u8>>(&self) -> ParseSigned<'_, T>;
}

pub trait WordParserIter {
    fn iter_lowercase(&self) -> ParseLowercase<'_>;
    fn iter_uppercase(&self) -> ParseUppercase<'_>;
}

impl ParserIter for &[u8] {
    fn to_unsigned<T: Num + Ten + From<u8>>(&self) -> T {
        to_unsigned(self)
    }
    
    fn try_signed<T: Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T> {
        try_signed(&mut self.iter())
    }
    
    fn try_unsigned<T: Num + Ten + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.iter())
    }


    fn iter_unsigned<T: Num + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.iter(), phantom: PhantomData }
    }

    fn iter_signed<T: Num + Signed + Ten + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.iter(), phantom: PhantomData }
    }
}

impl ParserIter for [u8] {
    fn to_unsigned<T: Num + Ten + From<u8>>(&self) -> T {
        to_unsigned(self)
    }
    
    fn try_signed<T: Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T> {
        try_signed(&mut self.iter())
    }
    
    fn try_unsigned<T: Num + Ten + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.iter())
    }


    fn iter_unsigned<T: Num + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.iter(), phantom: PhantomData }
    }

    fn iter_signed<T: Num + Signed + Ten + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.iter(), phantom: PhantomData }
    }
}

impl ParserIter for &str {
    fn to_unsigned<T: Num + Ten + From<u8>>(&self) -> T {
        to_unsigned(self.as_bytes())
    }
    
    fn try_signed<T: Signed + ConstZero + Ten + From<u8>>(&self) -> Option<T> {
        try_signed(&mut self.as_bytes().iter())
    }
    
    fn try_unsigned<T: Num + Ten + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.as_bytes().iter())
    }


    fn iter_unsigned<T: Num + Ten + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.as_bytes().iter(), phantom: PhantomData }
    }

    fn iter_signed<T: Signed + Ten + From<u8>>(&self) -> ParseSigned<'_, T> {
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

#[test]

fn test_iter_lowercase() {
    let input = "abcXb?cdAa";
    let output: Vec<_> = input.iter_lowercase().collect();
    assert_eq!(output, vec!("abc", "b", "cd", "a"));
}