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

ten!(u8 u16 u32 u64 u128 usize i16 i32 i64 i128);

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

pub trait UnsignedIter {
    fn next_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> Option<T>;
    fn next_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> Option<T>;
    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T>;
    fn iter_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> ParseSigned<'_, T>;

}

impl UnsignedIter for &str {
    fn next_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> Option<T> {
        next_signed(&mut self.bytes())  
    }
    
    fn next_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> Option<T> {
        next_unsigned(&mut self.bytes())  
    }


    fn iter_unsigned<T: Integer + Ten<T> + From<u8>>(&self) -> ParseUnsigned<'_, T> {
        ParseUnsigned { bytes: self.bytes(), phantom: PhantomData }
    }

    fn iter_signed<T: Integer + Signed + Ten<T> + From<u8>>(&self) -> ParseSigned<'_, T> {
        ParseSigned { bytes: self.bytes(), phantom: PhantomData }
    }
}

