pub struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words.iter() {
            if Self::is_palindromic(word) {
                return word.to_string();
            }
        }
        "".to_string()
    }

    fn is_palindromic(word: &str) -> bool {
        let n = word.len();

        let iter: Vec<u8> = word.bytes().collect();

        for i in 0..n / 2 {
            if iter[i] != iter[n - 1 - i] {
                return false
            }
        }
        true
    }
}