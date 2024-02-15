use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut d: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let mut a: Vec<char> = s.chars().collect();
            a.sort();
            let encode: String = a.into_iter().collect();

            let tmp = d.entry(encode).or_insert(vec![]);
            tmp.push(s)
        }

        d.into_values().collect()
    }
}

// impl Solution {
//     pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
//         let mut anagrams = HashMap::new();

//         for str in strs {
//             let mut chars: Box<[_]> = str.chars().collect();
//             chars.sort_unstable();

//             match anagrams.get_mut(&chars) {
//                 None => {
//                     anagrams.insert(chars, vec![str]);
//                 }
//                 Some(list) => list.push(str),
//             }
//         }

//         anagrams.into_values().collect()
//     }
// }