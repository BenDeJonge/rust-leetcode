//! https://leetcode.com/problems/find-median-from-data-stream/
//! Hard - [two-pointers, design, sorting, heap-priority-queue, data-stream]
//!
//! The median is the middle value in an ordered integer list.
//! If the size of the list is even, there is no middle value,
//! and the median is the mean of the two middle values.
//! - For example, for arr = [2,3,4], the median is 3.
//! - For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
//!
//! Implement the MedianFinder class:
//! - MedianFinder() initializes the MedianFinder object.
//! - void addNum(int num) adds the integer num from the data stream
//!   to the data structure.
//! - double findMedian() returns the median of all elements so far.
//!   Answers within 10-5 of the actual answer will be accepted.
//!
//! Example 1:
//! Input
//! [&"MedianFinder&", &"addNum&", &"addNum&", &"findMedian&", &"addNum&", &"findMedian&"]
//! [[], [1], [2], [], [3], []]
//! Output
//! [null, null, null, 1.5, null, 2.0]
//! Explanation
//! MedianFinder medianFinder = new MedianFinder();
//! medianFinder.addNum(1);    // arr = [1]
//! medianFinder.addNum(2);    // arr = [1, 2]
//! medianFinder.findMedian(); // return 1.5 (i.e., (1 + 2) / 2)
//! medianFinder.addNum(3);    // arr[1, 2, 3]
//! medianFinder.findMedian(); // return 2.0
//!
//! Constraints:
//! - -105 <= num <= 105
//! - There will be at least one element in the data structure before calling findMedian.
//! - At most 5 * 104 calls will be made to addNum and findMedian.
//!
//! Follow up:
//! - If all integer numbers from the stream are in the range [0, 100],
//!   how would you optimize your solution?
//! - If 99% of all integer numbers from the stream are in the range [0, 100],
//!   how would you optimize your solution?

use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    ops::Add,
};

struct MedianFinder {
    /// A max-heap which maximum is:
    /// - for odd n: the median.
    /// - for even n: the low half of the median.
    low: BinaryHeap<i32>,
    /// A min-heap which minimum is:
    /// - for even n: the right half of the median.
    high: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            low: BinaryHeap::new(),
            high: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(low) = self.low.peek() {
            if num < *low {
                self.push_to_low(num);
            } else {
                self.push_to_high(num);
            }
        }
        // The first number, both heaps are still empty.
        else {
            self.low.push(num);
        }
    }

    /// The number is small enough to store in the low category.
    /// Through size balancing, the largest of the small numbers
    /// flows up into the high category.
    fn push_to_low(&mut self, num: i32) {
        self.low.push(num);
        if self.low.len() > self.high.len() + 1 {
            self.high.push(Reverse(self.low.pop().unwrap()));
        }
    }

    /// The number is too large to store in the low category.
    /// Through size balancing, the smallest of the large numbers
    /// flows back into the low category.
    fn push_to_high(&mut self, num: i32) {
        self.high.push(Reverse(num));
        if self.high.len() > self.low.len() {
            self.low.push(self.high.pop().unwrap().0);
        }
    }

    /// Because of the size balancing, there are only 2 scenarios:
    /// n(low) == n(high): the median is the average of both extremes.
    /// n(low) == n(high) + 1: the median is the largest of low.
    fn find_median(&self) -> f64 {
        match self.low.len().cmp(&self.high.len()) {
            Ordering::Equal => {
                self.low
                    .peek()
                    .unwrap_or(&0)
                    .add(self.high.peek().unwrap_or(&Reverse(0)).0) as f64
                    / 2.0
            }
            Ordering::Greater => *self.low.peek().unwrap() as f64,
            Ordering::Less => unreachable!("low should grow faster"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_leetcode::hard::s0295_find_median_from_data_stream::MedianFinder;

    const TOLERANCE: f64 = 10e-5;

    #[test]
    fn test_0295() {
        for (stream, expected) in [
            (vec![], 0.0),
            (vec![2, 2], 2.0),
            (vec![2, 2, 2], 2.0),
            (vec![2, 2, 0, 0, 2], 2.0),
            (vec![2, 2, 0, 0, 2, 0], 1.0),
            (vec![1, 2, 3], 2.0),
            (vec![-1, 2, -3], -1.0),
            (vec![1, 2, 3, 4], 2.5),
            (vec![-1, 2, -3, 4], 0.5),
        ] {
            helper(stream.into_iter(), expected);
        }
    }

    fn helper(stream: impl Iterator<Item = i32>, expected: f64) {
        let mut finder = MedianFinder::new();
        for num in stream {
            finder.add_num(num);
        }
        let median = finder.find_median();
        assert!((median - expected).abs() < TOLERANCE);
    }
}
