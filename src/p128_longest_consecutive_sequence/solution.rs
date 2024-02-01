use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {

        let set: HashSet<i32> = nums.iter().cloned().collect();
        let mut res = 0;

        for num in set.iter() {
            let mut cur = *num;
            if set.contains(&(cur - 1)) {
                continue
            }

            let mut length = 1;
            while set.contains(&(cur + 1)) {
                length += 1;
                cur += 1;
            }
            res = res.max(length);
        }

        return res;
    }
}