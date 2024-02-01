pub struct Solution {}

const MOD: i64 = 1000000007;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = vec![];
        let mut res: i64 = 0;

        for i in 0..=arr.len() {
            while !stack.is_empty() && (i == arr.len() || arr[stack[stack.len() - 1] as usize] >= arr[i]) {
    
                let mid = stack.pop().unwrap() as i64;
                let left = match stack.len() {
                    0 => -1,
                    _ => stack[stack.len() - 1] as i64
                };
                let right = i as i64;

                let count = (mid - left) * (right - mid) % MOD;
                res = (res + count * arr[mid as usize] as i64) % MOD;
        
            }
            stack.push(i);
        }

        res as i32
    }
}