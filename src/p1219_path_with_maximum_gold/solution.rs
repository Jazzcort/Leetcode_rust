use std::collections::HashSet;
pub struct Solution;
impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        fn helper(i: usize, j: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
            if grid[i][j] == 0 {
                return 0;
            }
            let mut cur = 0;
            let tmp = grid[i][j];
            grid[i][j] = 0;

            if let Some(pre_row) = i.checked_sub(1) {
                cur = cur.max(helper(pre_row, j, grid));
            }

            if let Some(pre_col) = j.checked_sub(1) {
                cur = cur.max(helper(i, pre_col, grid));
            }

            if i + 1 < grid.len() {
                cur = cur.max(helper(i + 1, j, grid));
            }

            if j + 1 < grid[0].len() {
                cur = cur.max(helper(i, j + 1, grid));
            }
            grid[i][j] = tmp;
            cur + grid[i][j]
        }
        
        let mut res = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                res = res.max(helper(i, j, &mut grid));
            }
        }
        
        res
    }
}