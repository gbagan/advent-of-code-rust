use num_traits::*;

pub trait Bits<T> {
    fn bit_iterator(self) -> BitsIterator<T>;
}

impl<T> Bits<T> for T
where
    T: Zero,
{
    fn bit_iterator(self) -> BitsIterator<T> {
        BitsIterator { n: self }
    }
}

pub struct BitsIterator<T> {
    n: T,
}

impl<T> Iterator for BitsIterator<T>
where
    T: PrimInt + ConstOne,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.n.is_zero() {
            return None;
        }

        let t = self.n.trailing_zeros() as usize;
        self.n = self.n ^ (T::ONE << t);

        Some(t)
    }
}