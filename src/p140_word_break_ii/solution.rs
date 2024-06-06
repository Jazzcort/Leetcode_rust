use std::collections::HashSet;

pub struct Solution;
// impl Solution {
//     pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
//         let dict: HashSet<String> = word_dict.into_iter().collect();
//         let mut res = vec![];

//         fn helper<'a>(start: usize, end: usize, dict: &HashSet<String>, cur: &mut Vec<&'a str>, res: &mut Vec<String>, s:  &'a str) {
            
//             if start >= s.len() {
//                 res.push(cur.as_slice().join(" "));
//                 return 
//             }

//             if end >= s.len() {
//                 return
//             }

//             let tmp = &s[start..=end];

//             if dict.contains(tmp) {
//                 cur.push(tmp);
//                 helper(end + 1, end + 1, dict, cur, res, s);
//                 cur.pop();
//             }

//             helper(start, end + 1, dict, cur, res, s);

//         }

//         helper(0, 0, &dict, &mut vec![], &mut res, &s);

//         res
//     }
// }

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict: HashSet<String> = word_dict.into_iter().collect();
        let mut res = vec![];

        fn helper<'a>(start: usize, dict: &HashSet<String>, cur: &mut Vec<&'a str>, res: &mut Vec<String>, s:  &'a str) {
            
            if start >= s.len() {
                res.push(cur.as_slice().join(" "));
                return 
            }

            for end in start + 1..=s.len() {
                let tmp = &s[start..end];
                if dict.contains(tmp) {
                    cur.push(tmp);
                    helper(end, dict, cur, res, s);
                    cur.pop();
                }
            }

        }

        helper(0, &dict, &mut vec![], &mut res, &s);

        res
    }
}