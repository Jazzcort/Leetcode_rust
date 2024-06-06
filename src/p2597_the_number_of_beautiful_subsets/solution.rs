use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut cnt = HashMap::new();
        let mut res = 1;

        for num in nums.into_iter() {
            let reminder = num % k;
            let handle = cnt.entry(reminder).or_insert(HashMap::new());
            let handle = handle.entry(num).or_insert(0);
            *handle += 1;
        } 

        for mut group in cnt.into_values() {
            let mut group: Vec<(i32, i32)> = group.drain().collect();
            group.sort_unstable();
            res *= Solution::count_subset(group, k)
        }

        res - 1
    }

    fn count_subset(group: Vec<(i32, i32)>, k: i32) -> i32 {
        let mut res = 1;
        let (mut pre1, mut pre2) = (1, 0);
        let n = group.len();

        for i in 0..n {
            let skip = pre1;

            let mut take = 2_i32.pow(group[i].1 as u32) - 1;

            if i.checked_sub(1) != None && group[i].0 - group[i - 1].0 == k {
                take *= pre2;
            } else {
                take *= pre1;
            }

            res = skip + take;
            (pre1, pre2) = (res, pre1)
        }

        res
    }
}