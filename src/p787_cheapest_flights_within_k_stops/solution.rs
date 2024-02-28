use std::collections::{HashSet, VecDeque};

pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut neigh: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        let mut s_path: Vec<i32> = Vec::from(vec![1_000_000_000; n as usize]);
        s_path[src as usize] = 0;


        for flight in flights.into_iter() {
            let (u, v, dis) = (flight[0], flight[1], flight[2]);
            neigh[u as usize].push((v, dis));
        }

        let mut que = VecDeque::from(vec![src]);
        let mut level = 0;

        while !que.is_empty() && level <= k {
            let n = que.len();

            let s_path_copy = s_path.clone();
            let mut seen: HashSet<i32> = HashSet::new();

            for _i in 0..n {
                let cur = que.pop_front().unwrap();
                let cost = s_path_copy[cur as usize];

                for (v, dis) in neigh[cur as usize].iter() {
            
                    let cur_cost = s_path[*v as usize];
                    if cost + dis < cur_cost {
                        s_path[*v as usize] = cur_cost.min(cost + dis);
                        if !seen.contains(&v) {
                            que.push_back(*v);
                            seen.insert(*v);
                        } 
                    }
                }
            }

            level += 1
        }

        if s_path[dst as usize] != 1_000_000_000 {
            s_path[dst as usize]
        } else {
            -1
        }
    }
}
