use std::collections::HashMap;
use std::cell::RefCell;
pub struct Solution {}

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {

        let d: RefCell<HashMap<(i32, i32, i32), i32>> = RefCell::new(HashMap::new());
        let dir: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        fn helper(m: &i32, n: &i32, left: i32, x: i32, y: i32, map: &RefCell<HashMap<(i32, i32, i32), i32>>, dir: &Vec<(i32, i32)>) -> i32 {
            if left == 0 {
                return 0;
            }

            if map.borrow().contains_key(&(left, x, y)) {
                return * (map.borrow().get(&(left, x, y)).unwrap());
            }

            let mut added = 0;

            for (dx, dy) in dir.iter() {
                let (nx, ny) = (x + dx, y + dy);

                if nx < 0 || nx >= *m || ny < 0 || ny >= *n {
                    added += 1;
                } else {
                    added += helper(m, n, left - 1, nx, ny, map, dir);
                    added %= MOD;
                }
            }

            map.borrow_mut().insert((left, x, y), added);
            added
        }

        helper(&m, &n, max_move, start_row, start_column, &d, &dir)
    }
}