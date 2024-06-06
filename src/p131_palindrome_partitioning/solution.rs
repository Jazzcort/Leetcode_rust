pub struct Solution;
// impl Solution {
//     pub fn partition(s: String) -> Vec<Vec<String>> {

//         let mut cur = vec![];
//         let mut res = vec![];
//         fn helper(start: usize, end: usize, s: &String, res: &mut Vec<Vec<String>>, cur: &mut Vec<String>) {
//             if start >= s.len() {
//                 res.push(cur.clone());
//             }

//             if end >= s.len() {
//                 return 
//             }

//             let tmp = &s[start..=end];

//             if Solution::check_palindrome(tmp) {
//                 cur.push(tmp.to_string());
//                 helper(end + 1, end + 1, s, res, cur);
//                 cur.pop();
//             }

//             helper(start, end + 1, s, res, cur);

//         }

//         helper(0, 0, &s, &mut res, &mut cur);

//         res

//     }

//     fn check_palindrome(word: &str) -> bool {
//         let arr: Vec<char> = word.chars().collect();
        
//         for ind in 0..arr.len() / 2 {
//             if arr[ind] != arr[arr.len() - 1 - ind] {
//                 return false;
//             }
//         }

//         true
//     }
// }

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {

        let mut cur = vec![];
        let mut res = vec![];
        fn helper(ind: usize, s: &String, res: &mut Vec<Vec<String>>, cur: &mut Vec<String>) {
            if ind >= s.len() {
                res.push(cur.clone());
                return
            }

            for end in ind..s.len() {
                let tmp = &s[ind..=end];

                if Solution::check_palindrome(tmp) {
                    cur.push(tmp.to_string());
                    helper(end + 1, s, res, cur);
                    cur.pop();
                }
            }
        }

        helper(0, &s, &mut res, &mut cur);
        res
    }

    fn check_palindrome(word: &str) -> bool {
        let arr: Vec<char> = word.chars().collect();
        
        for ind in 0..arr.len() / 2 {
            if arr[ind] != arr[arr.len() - 1 - ind] {
                return false;
            }
        }

        true
    }
}