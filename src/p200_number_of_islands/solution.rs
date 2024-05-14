pub struct Solution;
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {

        let mut res = 0;

        let dir = vec![(0, 1), (0, -1), (1, 0), (-1 ,0)];

        fn helper(i: i32, j: i32, ref_grid: &mut Vec<Vec<char>>, dir: &Vec<(i32, i32)>) {
            ref_grid[i as usize][j as usize] = '0';

            for (di, dj) in dir.iter() {
                let (ni, nj) = (i + di, j + dj);

                if ni < 0 || ni > ref_grid.len() as i32 - 1 || nj < 0 || nj > ref_grid[0].len() as i32 - 1 {
                    continue;
                }

                if ref_grid[ni as usize][nj as usize] == '1' {
                    helper(ni, nj, ref_grid, dir);
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    res += 1;
                    helper(i as i32, j as i32, &mut grid, &dir);
                }
            }
        }
        res
    }
}