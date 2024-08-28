use std::{cmp::Ordering, collections::BinaryHeap};

pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<MinWrapper<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn new() -> Self {
        MinHeap { heap: BinaryHeap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        MinHeap { heap: BinaryHeap::with_capacity(capacity) }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(MinWrapper { key, value });
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }
}

impl<K: Ord, V> Default for MinHeap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MinWrapper<K, V> {
    pub key: K,
    pub value: V,
}

impl<K: PartialEq, V> PartialEq for MinWrapper<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: PartialEq, V> Eq for MinWrapper<K, V> {}

impl<K: Ord, V> PartialOrd for MinWrapper<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for MinWrapper<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}