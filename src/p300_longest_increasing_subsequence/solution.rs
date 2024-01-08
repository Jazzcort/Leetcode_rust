use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut deque: VecDeque<i32> = VecDeque::new();

        for num in &nums {
            if deque.len() == 0 || *deque.get(deque.len() - 1).unwrap() < *num {
                deque.push_back(*num);
            } else {
                let ind = deque.binary_search(num);
                match ind {
                    Ok(_) => (),
                    Err(i) => {
                        *deque.get_mut(i).unwrap() = *num;
                    }
                }
            }
        }
        deque.len() as i32
    }

    pub fn length_of_lis_dp(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![1; nums.len()];
        let mut res = 1;

        for i in 0..nums.len() {
            for j in 0..i {
                if nums[i] > nums[j] {
                    *dp.get_mut(i).unwrap() = dp[i].max(dp[j] + 1)
                }
                
            }
            res = res.max(dp[i]);
        }
        res
    }
}