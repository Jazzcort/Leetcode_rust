pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut dp: Vec<i32> = vec![0; n];

        let mut pre1 = 0;

        for i in 0..n {
            let pre2 = if i < 2 {
                0
            } else {
                dp[i - 2]
            };

            dp[i] = pre1.max(pre2 + nums[i]);
            pre1 = dp[i];
        }
        
        return pre1
    }
}