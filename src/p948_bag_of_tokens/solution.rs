pub struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        let n = tokens.len();
        if n == 0 {
            return 0;
        }

        tokens.sort_unstable();

        let (mut i, mut j) = (0, 0);
        let mut cur_power = power;
        let mut score = 0;

        loop {
            while j < tokens.len() && tokens[j] <= cur_power  {
                cur_power -= tokens[j];
                j += 1;
            }

            score = score.max(j - i);

            if j == tokens.len() || j == i {
                break;
            } 

            cur_power += tokens.pop().unwrap();
            i += 1;
        }

        score as i32
    }
}