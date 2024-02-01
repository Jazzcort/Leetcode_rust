use std::collections::HashMap;
pub struct Solution {}

impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        for row in matrix.iter_mut() {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }

        let mut res = 0;
        let mut d: HashMap<i32, i32> = HashMap::new();

        for j in 0..matrix[0].len() {
            for dj in 0..j + 1 {
                let mut cur_sum = 0;
                d.clear();
                d.insert(0, 1);
                
                for row in matrix.iter() {
                    cur_sum += row[j] - if dj >= 1 { row[dj - 1] } else { 0 };
                    if d.contains_key(&(cur_sum - target)) {
                        res += d.get(&(cur_sum - target)).unwrap();
                    }

                    let tmp = d.entry(cur_sum).or_insert(0);
                    *tmp += 1;
                }
            }
        }
        
        res
    }
}