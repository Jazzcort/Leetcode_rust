use std::collections::HashMap;
pub struct Solution;
impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut letter_freq = vec![0; 26];
        for c in letters.into_iter() {
            let ind = c as u8 - 97;
            letter_freq[ind as usize] += 1
        }

        fn helper(ind: usize, letter_freq: &mut Vec<i32>, words: &Vec<String>, score: &Vec<i32>) -> i32{
            if ind >= words.len() {
                return 0;
            }

            let skip = helper(ind + 1, letter_freq, words, score);

            let mut tmp = HashMap::new();

            for c in words[ind].chars() {
                let handle = tmp.entry(c).or_insert(0);
                *handle += 1;
            }

            let map_iter: Vec<(char, i32)> = tmp.drain().collect();

            for (c, cnt) in map_iter.iter() {
                let i = (*c as u8 - 97) as usize;
                if *cnt > letter_freq[i] {
                    return skip;
                }
            }

            for (c, cnt) in map_iter.iter() {
                let i = (*c as u8 - 97) as usize;
                letter_freq[i] -= cnt;
            }

            let take = helper(ind + 1, letter_freq, words, score);

            for (c, cnt) in map_iter.into_iter() {
                let i = (c as u8 - 97) as usize;
                letter_freq[i] += cnt;
            }
            
            skip.max(take + Solution::count_score(score, &words[ind]))
        }

        helper(0, &mut letter_freq, &words, &score)
    }

    fn count_score(score: &Vec<i32>, word: &str) -> i32 {
        let mut res = 0;
        for c in word.chars() {
            let ind = (c as u8 - 97) as usize;
            res += score[ind];
        }
        res
    }
}