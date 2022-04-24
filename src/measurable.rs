pub trait Measurable {
    fn size(&self) -> usize;
}

use std::collections::HashMap;

impl Measurable for String {
    fn size(&self) -> usize {
        self.len()
    }
}

impl Measurable for str {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Measurable for [T] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Measurable for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<K, V> Measurable for HashMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}
