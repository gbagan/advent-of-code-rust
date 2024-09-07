use anyhow::*;
use std::iter::Enumerate;
use std::marker::PhantomData;
use std::str::Bytes;
use num_integer::Integer;
use num_traits::Signed;


pub trait Ten<T> {
    const TEN: T;
}

macro_rules! ten {
    ($($t:ty)*) => ($(
        impl Ten<$t> for $t {
            const TEN: $t = 10;
        }
    )*)
}

ten!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128);

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<&'a T>,
}

impl<T: Integer + Ten<T> + From<u8>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        next_unsigned(&mut self.bytes)
    }
}

pub struct ParseSigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<&'a T>,
}

impl<T: Integer + Signed + Ten<T> + From<u8>> Iterator for ParseSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        next_signed(&mut self.bytes)
    }
}


fn next_unsigned<T: Integer + Ten<T> + From<u8>>(bytes: &mut Bytes<'_>) -> Option<T> {
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


fn next_signed<T: Integer + Signed + Ten<T> + From<u8>>(bytes: &mut Bytes<'_>) -> Option<T> {
    let (mut n, negative) = loop {
        let byte = bytes.next()?;
        let digit = byte.wrapping_sub(b'0');

        if digit == 253 {
            break (T::zero(), true);
        }
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


fn next_lower_case_token<'a>(slice: &'a str, iter: &mut Enumerate<Bytes<'_>>) -> Option<&'a str> {
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
    bytes: Enumerate<Bytes<'a>>,
}

impl<'a> Iterator for ParseLowercase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        next_lower_case_token(self.slice, &mut self.bytes)
    }
}

fn next_upper_case_token<'a>(slice: &'a str, iter: &mut Enumerate<Bytes<'_>>) -> Option<&'a str> {
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
    bytes: Enumerate<Bytes<'a>>,
}

impl<'a> Iterator for ParseUppercase<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        next_upper_case_token(self.slice, &mut self.bytes)
    }
}




pub trait ParserIter {
    fn next_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> Result<T>;
    fn next_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> Result<T>;
    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> ParseSigned<'_, T>;
    fn iter_lowercase(&self) -> ParseLowercase<'_>;
    fn iter_uppercase(&self) -> ParseUppercase<'_>;
}

impl ParserIter for &str {
    fn next_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> Result<T> {
        next_signed(&mut self.bytes()).context("No integer found")
    }
    
    fn next_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> Result<T> {
        next_unsigned(&mut self.bytes()).context("No integer found")
    }


    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.bytes(), phantom: PhantomData }
    }

    fn iter_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.bytes(), phantom: PhantomData }
    }

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