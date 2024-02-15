pub struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut pos: Vec<i32> = vec![];
        let mut neg: Vec<i32> = vec![];
        let n = nums.len();

        for num in nums.into_iter() {
            if num > 0 {
                pos.push(num);
            } else {
                neg.push(num);
            }
        }

        let mut res = vec![0; 0];
        let (mut i, mut j) = (0, 0); 

        for ind in 0..n {
            if ind % 2 == 0 {
                res.push(pos[i]);
                i += 1;
            } else {
                res.push(neg[j]);
                j += 1;
            }
        }
        
        res
    }
}