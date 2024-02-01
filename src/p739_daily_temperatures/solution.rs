pub struct Solution {}

impl Solution {
    pub fn daily_temperatures(mut temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res: Vec<i32> = vec![0; n];
        let mut stck: Vec<(i32, i32)> = vec![];
        let mut ind = n as i32 - 1;

        while !temperatures.is_empty() {
            let cur = temperatures.pop().unwrap();

            while !stck.is_empty() && stck.last().unwrap().0 <= cur {
                stck.pop();
            }

            if !stck.is_empty() {
                res[ind as usize] = stck.last().unwrap().1 - ind;
            }

            stck.push((cur, ind));
            ind -= 1;
        }
        res
    }
}