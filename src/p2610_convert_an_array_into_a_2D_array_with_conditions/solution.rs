use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut d: HashMap<i32, i32> = HashMap::new();
        let mut maxi = 0;

        for num in nums {
            let tmp = d.entry(num).or_insert(0);
            *tmp += 1;
            maxi = maxi.max(*tmp);
        }

        let mut res: Vec<Vec<i32>> = Vec::new();

        for _i in 0..maxi {
            let mut tmp: Vec<i32> = Vec::new();
            for (key, count) in &mut d {
                if *count > 0 {
                    tmp.push(*key);
                    *count -= 1;
                }
            }
            res.push(tmp);
        }

        res

    }
}