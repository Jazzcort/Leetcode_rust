pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut left, mut quota): (Vec<usize>, Vec<usize>) = (vec![], vec![]);

        for (ind, c) in s.char_indices() {
            if c == '(' {
                left.push(ind);
            } else if c == ')' {
                if left.is_empty() && quota.is_empty() {
                    return false;
                } else if !left.is_empty() {
                    left.pop();
                } else {
                    quota.pop();
                }
            } else {
                quota.push(ind);
            }
        }

        while !left.is_empty() {
            if quota.is_empty() {
                return false;
            }

            if quota.pop().unwrap() > *left.last().unwrap() {
                left.pop();
            }
        }
        
        return true;
    }
}