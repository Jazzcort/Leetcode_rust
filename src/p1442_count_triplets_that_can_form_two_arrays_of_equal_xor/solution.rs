use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut res = 0;
        let mut dic: HashMap<i32, Vec<usize>> = HashMap::new();

        for (ind, num) in arr.into_iter().enumerate() {
            cur ^= num;

            if cur == 0 {
                res += ind as i32;
            }

            if dic.contains_key(&cur) {
                for i in dic.get(&cur).unwrap().iter() {
                    res += (ind - i - 1) as i32;
                }
            }

            let handle = dic.entry(cur).or_insert(vec![]);
            handle.push(ind);
        }

        res
    }
}