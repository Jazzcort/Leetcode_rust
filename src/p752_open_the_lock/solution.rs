use std::collections::{HashSet, VecDeque};

pub struct Solution;

// Faster solution only use vec
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut seen = [false; 10_000];
        for de in deadends.into_iter().map(|n| n.parse::<usize>().unwrap()) {
            seen[de] = true;
        }

        let target = target.parse::<usize>().unwrap();

        let mut moves = 0;

        let mut q = std::collections::VecDeque::from([0]);
        while !q.is_empty() {
            for _ in 0..q.len() {
                let code = q.pop_front().unwrap();

                if code == target {
                    return moves;
                }

                if seen[code] { continue; }
                seen[code] = true;

                for place in &[1, 10, 100, 1000] {
                    let digit = (code / place) % 10;

                    let code_without_digit = code - (place * digit);

                    let inc = code_without_digit + place * ((digit + 1) % 10);
                    q.push_back(inc);
                    let dec = code_without_digit + place * ((digit + 9) % 10);
                    q.push_back(dec);
                }
            }
            moves += 1;
        }
        -1
    }
}

// With set 
// impl Solution {
//     pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
//         if "0000" == target {
//             return 0;
//         }

//         let mut forbiden: HashSet<String> = deadends.into_iter().collect();

//         if forbiden.contains("0000") {
//             return -1;
//         }

//         let mut que: VecDeque<String> = VecDeque::from(vec!["0000".to_string()]);
//         let mut turns = 0;

//         while !que.is_empty() {
//             let n = que.len();

//             for _ in 0..n {
//                 let cur_str = que.pop_front().unwrap();
//                 let cur: Vec<char> = cur_str.chars().collect();
//                 for ind in 0..cur.len() {
//                     let cur_digit = cur[ind] as u8 - 48;
//                     let (last_digit, next_digit) = ((cur_digit + 9) % 10, (cur_digit + 1) % 10);

//                     let mut tmp = cur.clone();
//                     tmp[ind] = (last_digit + 48) as char;
//                     let candidate1: String = tmp.into_iter().collect();
//                     if candidate1 == target {
//                         return turns + 1;
//                     }

//                     let mut tmp = cur.clone();
//                     tmp[ind] = (next_digit + 48) as char;
//                     let candidate2: String = tmp.into_iter().collect();
//                     if candidate2 == target {
//                         return turns + 1;
//                     }

//                     if !forbiden.contains(&candidate1) {
//                         que.push_back(candidate1.clone());
//                         forbiden.insert(candidate1);
//                     }

//                     if !forbiden.contains(&candidate2) {
//                         que.push_back(candidate2.clone());
//                         forbiden.insert(candidate2);
//                     }

                
//                 }
//             }
//             turns += 1;
//         }

//         return -1
//     }
// }