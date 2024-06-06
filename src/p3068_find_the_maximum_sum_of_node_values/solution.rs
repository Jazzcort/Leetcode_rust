pub struct Solution;
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let total = nums.iter().fold(0_i64, |acc, x| {
            return acc + *x as i64;
        });
        let mut total_diff = 0_i64;
        let mut positive_count = 0;
        let mut min_abs_diff = i64::MAX;

        for num in nums.into_iter() {
            let diff = (num ^ k) - num;

            if diff > 0 {
                positive_count += 1;
                total_diff += diff as i64;
            }

            min_abs_diff = min_abs_diff.min(diff.abs() as i64);
        }

        if positive_count % 2 == 1 {
            total_diff -= min_abs_diff;
        }

        total + total_diff
    }
}