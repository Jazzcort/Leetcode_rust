pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stck: Vec<usize> = vec![];
        let mut str_lst: Vec<char> = s.chars().collect();

        for (ind, c) in s.char_indices() {
            if c == '(' {
                stck.push(ind)
            } else if c == ')' {
                if !stck.is_empty() {
                    stck.pop();
                } else {
                    str_lst[ind] = ' ';
                }
            }
        }

        for ind in stck {
            str_lst[ind] = ' ';
        }

        let res: String = str_lst.into_iter().filter(|x| *x != ' ').collect();
        return res
    }
}