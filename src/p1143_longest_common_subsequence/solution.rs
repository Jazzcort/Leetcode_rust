pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());

        let mut dp = vec![0; m + 1];

        for i in 1..(n + 1) {
            let mut pre = 0;
            for j in 1..(m + 1) {
                let cur = dp[j];

                if t1[j - 1] == t2[i - 1] {
                    dp[j] = pre + 1;
                } else {
                    dp[j] = dp[j - 1].max(cur);
                }

                pre = cur;
            }
        }
        dp.pop().unwrap()
    }
}