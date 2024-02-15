use std::iter::Sum;

pub struct Solution {}

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        let mut summ: i64 = nums.iter().map(|num| *num as i64).sum();
        nums.sort_unstable();

        while !nums.is_empty() {
            let cur = nums.pop().unwrap() as i64;
            summ -= cur;
            if cur < summ {
                return summ + cur;
            }
        }
        
        -1
    }
}