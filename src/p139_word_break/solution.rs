use std::collections::{HashSet, HashMap};
pub struct Solution;
// impl Solution {
//     pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
//         let mut dp = vec![false; s.len()];

//         for i in 0..s.len() {
//             for word in word_dict.iter() {
//                 if word.len() > i + 1 {
//                     continue;
//                 } else if word.len() == i + 1 {
//                     if &s[..=i] == word {
//                         dp[i] = true;
//                         break;
//                     }
//                 } else {
//                     if &s[i - word.len() + 1..=i] == word && dp[i - word.len()] {
//                         dp[i] = true;
//                         break;
//                     }
//                 }
//             }
//         }

//         *dp.last().unwrap()
//     }
// }

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let mut memo = HashMap::new();

        fn helper(start: usize, dict: & HashSet<String>, s: &str, memo: &mut HashMap<usize, bool>) -> bool {

            if start >= s.len() {
                return true
            }

            if memo.contains_key(&start) {
                return *memo.get(&start).unwrap()
            }

            for i in start + 1..=s.len() {
                let cur = &s[start..i];
                if dict.contains(cur) {
                    if helper(i, dict, s, memo) {
                        memo.insert(start, true);
                        return true
                    }
                }
            }

            memo.insert(start, false);

            false
        }

        helper(0, &dict, &s, &mut memo)
    }
}
