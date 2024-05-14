pub struct Solution;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut score_tuple = vec![];

        for (ind, score) in score.into_iter().enumerate() {
            score_tuple.push((score, ind));
        }

        score_tuple.sort_unstable_by_key(|(score, _)| -*score);

        let mut res = vec!["".to_string(); score_tuple.len()];

        for (rank, (_, ind)) in score_tuple.into_iter().enumerate() {
            res[ind] = match rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => format!("{}", rank + 1)
            }
        }

        res
    }
}