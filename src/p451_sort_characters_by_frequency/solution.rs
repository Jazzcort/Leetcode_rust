use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut d: HashMap<char, usize> = HashMap::new();

        for c in s.chars() {
            let tmp = d.entry(c).or_insert(0);
            *tmp += 1;
        }

        let mut arr: Vec<(usize, char)> = vec![];

        for (key, val) in d.into_iter() {
            arr.push((val, key));
        }

        arr.sort_unstable();

        let mut res = String::new();

        while !arr.is_empty() {
            let (count, c) = arr.pop().unwrap();
            let tmp = c.to_string();
            res.push_str(&tmp.repeat(count));
        }

        res
    }
}