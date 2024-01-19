use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

        let (mut win_seen, mut lose_seen): (HashSet<i32>, HashSet<i32>) = (HashSet::new(), HashSet::new());
        let mut never_lose: HashSet<i32> = HashSet::new();
        let mut lose_once: HashSet<i32> = HashSet::new();

        for a in &matches {

            let winner = a[0];
            let loser = a[1];

            if !win_seen.contains(&winner) && !lose_seen.contains(&winner) {
                win_seen.insert(winner);
                never_lose.insert(winner);
            }

            if !lose_seen.contains(&loser) {
                lose_seen.insert(loser);
                lose_once.insert(loser);
            } else {
                lose_once.remove(&loser);
            }

            never_lose.remove(&loser);

        }

        let mut never_lose: Vec<i32> = never_lose.into_iter().collect();
        let mut lose_once: Vec<i32> = lose_once.into_iter().collect();
        never_lose.sort_unstable();
        lose_once.sort_unstable();

        return vec![never_lose, lose_once];
    }
}