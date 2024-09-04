use std::ops::{Range, RangeInclusive};
use std::thread;
use std::sync::atomic::{AtomicI32, AtomicU32, AtomicU64, AtomicUsize, Ordering};

struct Shared<A> {
    counter: AtomicUsize,
    result: A,
}

pub trait ParallelIterator: Sized + Send {
    type Item: Send;
    
    fn get(&self, index: usize) -> Self::Item;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    fn map<F, A>(self, f: F) -> Map<Self, F>
    where
        F: Fn(Self::Item) -> A + Sync + Send,
        A: Send,
    {
        Map::new(self, f)
    }

}

macro_rules! parallel_iterator {
    ($name:tt, $item:ty, $atomic: tt) => {
        pub trait $name: ParallelIterator<Item=$item> {
            fn reduce<R>(&self, init: $item, reduce: R) -> $item
            where
                Self: Sized + Send + Sync,
                R: Fn($item, $item)-> $item + Sync + Send
            {
                let shared = Shared { counter: AtomicUsize::new(self.start()), result: $atomic::new(init) };
                thread::scope(|scope| {
                    for _ in 0..thread::available_parallelism().unwrap().get() {
                        scope.spawn(|| {
                            loop {
                                let i = shared.counter.fetch_add(1, Ordering::Relaxed);
                                if i >= self.end() {
                                    break;
                                }
                                let res = self.get(i);
                                let _ = shared.result.fetch_update(Ordering::Relaxed,
                                                                  Ordering::Relaxed,
                                                                  |x| Some(reduce(x, res)));                
                            }
                        });
                    }
                });
                shared.result.load(Ordering::Relaxed)
            }

            fn sum(&self) -> $item
            where
                Self: Sized + Send + Sync,
            {
                self.reduce(0, |x, y| x+y)
            }
        }

        impl<I: ParallelIterator<Item=$item>> $name for I {}
    }
}

parallel_iterator!(ParallelIteratorI32, i32, AtomicI32);
parallel_iterator!(ParallelIteratorU32, u32, AtomicU32);
parallel_iterator!(ParallelIteratorU64, u64, AtomicU64);
parallel_iterator!(ParallelIteratorUSize, usize, AtomicUsize);











pub trait IntoParallelIterator {
    type Iter: ParallelIterator<Item = Self::Item>;
    type Item: Send;

    fn into_par_iter(self) -> Self::Iter;
}


// slice

pub struct SliceIter<'a, T: Sync> {
    slice: &'a [T],
}

impl<'a, T: Send + Sync + 'a> IntoParallelIterator for &'a [T] {
    type Item = &'a T;
    type Iter = SliceIter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        SliceIter { slice: self }
    }
}

impl<'a, T: Send + Sync> ParallelIterator for SliceIter<'a, T>
where
{
    type Item = &'a T;
    
    fn get(&self, index: usize) -> Self::Item {
        &self.slice[index]
    }

    fn start(&self) -> usize {
        0
    }

    fn end(&self) -> usize {
        self.slice.len()
    }
}

// vec

impl<'a, T: Send + Sync + 'a> IntoParallelIterator for &'a Vec<T> {
    type Item = &'a T;
    type Iter = SliceIter<'a, T>;

    fn into_par_iter(self) -> Self::Iter {
        <&[T]>::into_par_iter(self)
    }
}

// ranges

pub struct RangeIter<T: Sync> {
    range: Range<T>,
}

impl IntoParallelIterator for Range<usize> {
    type Item = usize;
    type Iter = RangeIter<usize>;

    fn into_par_iter(self) -> Self::Iter {
        RangeIter { range: self }
    }
}

impl ParallelIterator for RangeIter<usize>
where
{
    type Item = usize;
    
    fn get(&self, index: usize) -> Self::Item {
        index
    }

    fn start(&self) -> usize {
        self.range.start
    }

    fn end(&self) -> usize {
        self.range.end
    }
}

// inclusive ranges

pub struct RangeInclusiveIter<T: Sync> {
    range: RangeInclusive<T>,
}

impl IntoParallelIterator for RangeInclusive<usize> {
    type Item = usize;
    type Iter = RangeInclusiveIter<usize>;

    fn into_par_iter(self) -> Self::Iter {
        RangeInclusiveIter { range: self }
    }
}

impl ParallelIterator for RangeInclusiveIter<usize>
where
{
    type Item = usize;
    
    fn get(&self, index: usize) -> Self::Item {
        index
    }

    fn start(&self) -> usize {
        *self.range.start()
    }

    fn end(&self) -> usize {
        self.range.end() + 1
    }
}



// map

pub struct Map<I: ParallelIterator, F> {
    base: I,
    f: F,
} 

impl<I, F> Map<I, F>
where
    I: ParallelIterator,
{
    fn new(base: I, f: F) -> Self {
        Map { base, f }
    }
}

impl<I, F, A> ParallelIterator for Map<I, F>
where
    I: ParallelIterator,
    F: Fn(I::Item) -> A + Sync + Send,
    A: Send,
{
    type Item = A;
    
    fn get(&self, index: usize) -> Self::Item {
        (self.f)(self.base.get(index))
    }

    fn start(&self) -> usize {
        self.base.start()
    }

    fn end(&self) -> usize {
        self.base.end()
    }
}