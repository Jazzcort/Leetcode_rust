pub struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut pre = 0;
        let mut res = 0;

        for s in bank {
            let mut ct = 0;
            for ptr in s.as_bytes() {
                if *ptr == b'1' {
                    ct += 1;
                }
            }
            if ct != 0 {
                res += pre * ct;
                pre = ct;
            }
            
        }

        res
    }
}