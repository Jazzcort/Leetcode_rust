use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let mut cnt = HashMap::new();

        for num in hand.into_iter() {
            let handle = cnt.entry(num).or_insert(0);
            *handle += 1;
        }

        let mut hand_set: Vec<i32> = cnt.keys().cloned().collect();
        hand_set.sort_unstable();

        for num in hand_set.into_iter() {
            let cur = *cnt.get(&num).unwrap();

            if cur != 0 {
                for i in 1..group_size {
                    let tmp = num + i;
                    if cnt.contains_key(&tmp) && *cnt.get(&tmp).unwrap() >= cur {
                        let handle = cnt.get_mut(&tmp).unwrap();
                        *handle -= cur;
                    } else {
                        return false;
                    }
                }
            }
        }
        true        
    }
}