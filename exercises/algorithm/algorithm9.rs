/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

// [#derive(Debug)]
#[derive(Debug)]
pub struct Heap<T>
where
    T: Default + Ord + std::fmt::Display + std::fmt::Debug,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Display + std::fmt::Debug,
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
        //TODO
        self.items.push(value);
        self.count += 1;
        println!(
            "添加新元素:{:?}, 目前有{}个元素, 当前堆为{:?}",
            self.items.last(),
            self.count,
            self.items
        );

        let mut index = self.count;
        let mut parent_idx = self.parent_idx(index);
        while parent_idx > 0 && (self.comparator)(&self.items[index], &self.items[parent_idx]) {
            println!(
                "index:{}, parent_idx:{}, 当前堆为:{}",
                index, parent_idx, ' '
            );
            self.items.swap(index, parent_idx);

            index = self.parent_idx(index);
            parent_idx = self.parent_idx(index);
        }
        // self.items.push(value);
        // self.count += 1;

        // let mut idx = self.count;
        // let mut parent_idx = self.parent_idx(idx);
        // while parent_idx > 0 && (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
        //     self.items.swap(idx, parent_idx);
        //     idx = parent_idx;
        //     parent_idx = self.parent_idx(idx);
        // }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn two_children_present(&self, idx: usize) -> bool {
        self.right_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.right_child_idx(idx) <= self.count {
            if self.items[self.left_child_idx(idx)] > self.items[self.right_child_idx(idx)] {
                self.right_child_idx(idx)
            } else {
                self.left_child_idx(idx)
            }
        } else {
            self.left_child_idx(idx)
        }
        // unimplemented!()
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + std::fmt::Display + std::fmt::Debug,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Ord + std::fmt::Display + std::fmt::Debug,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        // if self.is_empty() {
        //     None
        // } else {
        //     self.items.swap(1, self.count);
        //     self.count -= 1;
        //     let ret = self.items.pop();

        //     let mut idx = 1;
        //     let mut left = self.left_child_idx(idx);
        //     let mut right = self.right_child_idx(idx);

        //     while left <= self.count {
        //         if right >= self.items.len() {
        //             if (self.comparator)(&self.items[idx], &self.items[left]) {
        //                 break;
        //             }
        //             self.items.swap(idx, left);
        //             idx = left;
        //         } else if (self.comparator)(&self.items[left], &self.items[right]) {
        //             self.items.swap(idx, left);
        //             idx = left;
        //         } else {
        //             self.items.swap(idx, right);
        //             idx = right;
        //         }
        //         left = self.left_child_idx(idx);
        //         right = self.right_child_idx(idx);
        //     }
        //     ret
        // }
        //TODO
        //     use std::time::{Duration, Instant};
        //     let start_time = Instant::now();
        //     let current_time = Instant::now();
        //     // 计算程序运行时间（秒）
        //     let elapsed_seconds = current_time.duration_since(start_time).as_secs();
        //     // 打印当前时间
        //     println!("Current time: {} seconds", elapsed_seconds);
        //     // 休眠1秒
        //     std::thread::sleep(Duration::from_secs(1));

        if self.is_empty() {
            return None;
        }
        println!("before:  {:?}", self.items);
        let length = self.len();
        self.items.swap(1, length);
        println!("swap:  {:?}", self.items);
        let option_t = self.items.pop();
        self.count -= 1;
        println!("option_t: {:?} {:?}", option_t, self.items);
        let mut idx = 1;

        // self.smallest_child_idx(index);
        while self.children_present(idx) {
            //左子树
            println!("idxidxidxidxidx: {:?} {:?}", idx, self.items);
            let left = self.left_child_idx(idx);

            if self.two_children_present(idx) {
                let right = self.right_child_idx(idx);
                println!("left:{} right:{}", left, right);
                if (self.comparator)(&self.items[left], &self.items[right]) {
                    if (self.comparator)(&self.items[left], &self.items[idx]) {
                        self.items.swap(left, idx);
                        idx = left;
                    } else {
                        break;
                    }
                } else {
                    if (self.comparator)(&self.items[right], &self.items[idx]) {
                        self.items.swap(right, idx);
                        idx = right;
                    } else {
                        break;
                    }
                }
            } else {
                if (self.comparator)(&self.items[left], &self.items[idx]) {
                    self.items.swap(left, idx);
                    idx = left;
                } else {
                    break;
                }
            }
        }
        // let length = self.len();
        // self.items.swap(idx, length - 1);
        // self.items[index] = self.items[self.len() - 1];
        option_t
    }
}

#[derive(Debug)]
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Display + std::fmt::Debug,
    {
        Heap::new(|a, b| a < b)
    }
}

#[derive(Debug)]
pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::fmt::Display + std::fmt::Debug,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
