use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {

        
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); nums.len()];
        let mut res = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let tmp = dp[i].entry(diff).or_insert(0);
                *tmp += 1;

                if dp[j].contains_key(&diff) {
                    let pre =  dp[j][&diff];
                    let tmp = dp[i].get_mut(&diff).unwrap();
                    *tmp += pre;
                    res += pre;
                }

            }
        }
        res
    }
}