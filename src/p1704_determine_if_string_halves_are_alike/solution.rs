use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        
        let s: String = s.to_lowercase();
        let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let (a, b) = s.split_at(s.len() / 2);

        let mut count_a = 0;
        let mut count_b = 0;

        for c in a.chars() {
            if vowels.contains(&c) {
                count_a += 1;
            }
        }

        for c in b.chars() {
            if vowels.contains(&c) {
                count_b += 1;
            }
        }
        
        count_a == count_b
    }
}