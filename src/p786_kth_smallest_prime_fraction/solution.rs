use std::collections::BinaryHeap;
use std::cmp::Reverse;
// use std::cmp::Ord;

#[derive(Debug, Eq)]
struct Tup {
    a: i32,
    b: i32,
    ind_a: usize,
    ind_b: usize
}

pub struct Solution;
impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut h = BinaryHeap::new();

        for ind in 1..arr.len() {
            h.push(Reverse(Tup{
                a: arr[0], 
                b: arr[ind],
                ind_a: 0,
                ind_b: ind
            }));
        }

        while k > 1 {
            let cur: Tup = h.pop().unwrap().0;

            if cur.ind_a < cur.ind_b - 1 {
                h.push(Reverse(Tup{
                    ind_a: cur.ind_a + 1,
                    ind_b: cur.ind_b,
                    a: arr[cur.ind_a + 1],
                    b: arr[cur.ind_b]
                }));
            }

            k -= 1;
        }

        let res = h.pop().unwrap().0;

        vec![res.a, res.b]
        
    }
}

impl Ord for Tup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_res = self.a * other.b;
        let other_res = other.a * self.b;

        if self_res > other_res {
            std::cmp::Ordering::Greater
        } else if self_res < other_res {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl PartialOrd for Tup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tup {
    fn eq(&self, other: &Self) -> bool {
        (self.a, self.b) == (other.a, other.b)
    }
}

