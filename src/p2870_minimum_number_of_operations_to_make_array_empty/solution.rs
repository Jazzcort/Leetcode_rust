use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut d: HashMap<i32, i32> = HashMap::new();

        for i in &nums {
            let tmp = d.entry(*i).or_insert(0);
            *tmp += 1;
        }

        let mut res = 0;

        for key in d.keys() {
            let num = d.get(key).unwrap();

            if *num == 1 {
                return -1;
            }

            if *num % 3 == 0 {
                res += *num / 3;
            } else {
                res += (*num / 3) + 1;
            }
        }

        res
    }
}