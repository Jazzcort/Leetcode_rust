use std::collections::VecDeque;
pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {

        if n == 1 {
            return vec![0];
        }

        let mut adj: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut degree: Vec<i32> = vec![0; n as usize];

        for pair in edges.into_iter() {
            let (u, v) = (pair[0], pair[1]);
            let (u, v) = (u as usize, v as usize);
            adj[u].push(v);
            adj[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        let mut que: VecDeque<usize> = VecDeque::new();
        for (ind, de) in degree.iter().enumerate() {
            if *de == 1 {
                que.push_back(ind);
            }
        }

        let mut remain = n;

        while remain > 2 {
            let t = que.len();
            remain -= t as i32;

            for _ in 0..t {
                let cur = que.pop_front().unwrap();
                for v in adj[cur].iter() {
                    degree[*v] -= 1;
                    if degree[*v] == 1 {
                        que.push_back(*v);
                    }
                } 
            }
        }

        let que:Vec<i32> = que.into_iter().map(|x| x as i32).collect();
        que
    }
}