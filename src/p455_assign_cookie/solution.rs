pub struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();

        let mut res = 0;

        while g.len() > 0 && s.len() > 0 {
            if g.pop().unwrap() <= s[s.len() -1] {
                res += 1;
                s.pop();
            }
            
        }

        res
    }
}