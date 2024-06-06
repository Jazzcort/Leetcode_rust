
pub struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor_sum = nums.iter().fold(0, |acc, num| { acc ^ *num });

        let right_most_one = xor_sum & (-xor_sum);

        let mut res = vec![0; 2];

        for num in nums.into_iter() {
            if num & right_most_one == 0 {
                res[0] ^= num;
            } else {
                res[1] ^= num;
            }
        }

        res
    }
}