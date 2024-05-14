pub struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut arr: [u8; 26] = [0; 26];
        let letters = "abcdefghijklmnopqrstuvwxyz";

        for c in s.as_bytes() {
            arr[(*c - b'a') as usize] += 1;
        }

        let mut res = String::new();
        
        for c in order.as_bytes() {
            let ind = (*c - b'a') as usize;
            let c_s = letters[ind..ind + 1].to_string();
            res.push_str(&c_s.repeat(arr[ind] as usize));
            arr[ind] = 0;
        }

        for (ind, c) in letters.chars().enumerate() {
            if arr[ind] > 0 {
                res.push_str(&c.to_string().repeat(arr[ind] as usize))
            }
        }
        
        res
    }
}