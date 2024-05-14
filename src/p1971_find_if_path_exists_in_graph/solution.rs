pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct DSU {
    arr: Vec<usize>,
    rank: Vec<usize>
}

impl DSU {
    fn search(&self, ind: usize) -> usize {
        if ind != self.arr[ind] {
            return self.search(self.arr[ind]);
        }

        ind
    }

    fn union(&mut self, x: usize, y: usize) {
        let (rx, ry) = (self.search(x), self.search(y));

        if rx != ry {
            if self.rank[rx] > self.rank[ry] {
                self.arr[ry] = rx;
                self.rank[rx] += self.rank[ry];
            } else {
                self.arr[rx] = ry;
                self.rank[ry] += self.rank[rx];
            }
        }

    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut dsu = DSU{
            arr: (0..n as usize).collect(),
            rank: vec![1; n as usize]
        };

        for pair in edges.into_iter() {
            let (u, v) = (pair[0], pair[1]);
            dsu.union(u as usize, v as usize);
        }
        
        dsu.search(source as usize) == dsu.search(destination as usize)
    }
}

// BFS
// impl Solution {
//     pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {

//         if source == destination {
//             return true;
//         }

//         let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();

//         for i in 0..n {
//             adj.insert(i, vec![]);
//         }

//         for pair in edges.iter() {
//             let (u, v) = (pair[0], pair[1]);
//             let handle = adj.get_mut(&u).unwrap();
//             handle.push(v);
//             let handle = adj.get_mut(&v).unwrap();
//             handle.push(u);
//         }

//         let mut que = VecDeque::from(vec![source]);
//         let mut seen: HashSet<i32> = HashSet::new();

//         while !que.is_empty() {
//             let n = que.len();

//             for _ in 0..n {
//                 let cur = que.pop_front().unwrap();
//                 for v in adj.get(&cur).unwrap() {
//                     if !seen.contains(v) {
//                         if *v == destination {
//                             return true;
//                         }
//                         que.push_back(*v);
//                         seen.insert(*v);
//                     }
//                 }
//             }
//         }

//         false
//     }
// }