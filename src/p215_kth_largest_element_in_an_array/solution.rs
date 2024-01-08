use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut h: BinaryHeap<i32> = BinaryHeap::from(nums);

        let mut tmp = k;

        while tmp > 1 {
            h.pop();
            tmp -= 1;
        }

        *h.peek().unwrap()
    }
}