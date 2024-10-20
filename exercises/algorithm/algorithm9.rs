use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;

        while idx > 1 {
            let parent_idx = self.parent_idx(idx);  // 先计算 parent_idx
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);  // 只进行一次可变借用
            }
            idx = parent_idx;
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let root = self.items.swap_remove(1);
        self.count -= 1;

        if self.count > 0 {
            let mut idx = 1;
            while self.left_child_idx(idx) <= self.count {
                let child_idx = self.smallest_child_idx(idx);
                if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                    self.items.swap(idx, child_idx);
                }
                idx = child_idx;
            }
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = Heap::new_min();
        heap.add(3);
        heap.add(1);
        heap.add(2);

        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(3));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = Heap::new_max();
        heap.add(3);
        heap.add(1);
        heap.add(2);

        assert_eq!(heap.next(), Some(3));
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
    }
}
