pub trait AOCIter: Iterator {
    #[inline]
    fn count_by<P>(self, predicate: P) -> usize
    where
        Self: Sized,
        P: Fn(Self::Item) -> bool,
    {
        let mut counter = 0;
        for x in self {
            if predicate(x) {
                counter += 1;
            }
        }
        counter
    }
}

impl<I: Iterator> AOCIter for I {}