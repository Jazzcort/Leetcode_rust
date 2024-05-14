use std::collections::VecDeque;

pub struct Solution;
impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort_unstable();

        let mut que: VecDeque<usize> = (0..deck.len()).collect();
        let mut res_deck = vec![0; deck.len()];

        for card in deck.into_iter() {
            res_deck[que.pop_front().unwrap()] = card;
            if !que.is_empty() {
                let tmp = que.pop_front().unwrap();
                que.push_back(tmp);
            }
        }

        res_deck
    }
}