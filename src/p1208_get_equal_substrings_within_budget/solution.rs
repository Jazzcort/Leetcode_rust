pub struct Solution;
impl Solution {
    pub fn equal_substring(s: String, t: String, mut max_cost: i32) -> i32 {

        let slice_s = s.as_bytes();
        let slice_t = t.as_bytes();
        
        let mut res = 0;
        let mut start = 0;

        for i in 0..s.len() {
            let diff = slice_s[i].abs_diff(slice_t[i]) as i32;

            while max_cost < diff {
                max_cost += slice_s[start].abs_diff(slice_t[start]) as i32;
                start += 1;
            }

            max_cost -= diff;
            res = res.max(i - start + 1); 
        }

        res as i32        
    }
}