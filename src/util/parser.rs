use std::marker::PhantomData;
use std::str::Bytes;
use num_integer::Integer;

pub struct ParseUnsigned<'a, T> {
    bytes: Bytes<'a>,
    phantom: PhantomData<&'a T>,
}

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

ten!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128);

impl<T: Integer + Ten<T> + From<u8>> Iterator for ParseUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        next_unsigned(&mut self.bytes)
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

pub trait ParseIter {
    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T>;
}

impl ParseIter for &str {
    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.bytes(), phantom: PhantomData }
    }
}