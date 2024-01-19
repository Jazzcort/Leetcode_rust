pub struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = matrix[0].clone();
        let n = matrix.len();

        for i in 1..n {
            let tmp = dp.clone();
            for j in 0..n {
                let left = match j {
                    0 => 0,
                    _ => j - 1
                };

                let right = (n - 1).min(j + 1);

                dp[j] = matrix[i][j] + tmp[left].min(tmp[j]).min(tmp[right]);
            }
        }

        let mut res = i32::MAX;
        for num in dp.drain(0..) {        
            res = res.min(num);
        }
        
        res
    }
}