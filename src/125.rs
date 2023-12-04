struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s.chars().filter(|c| c.is_alphanumeric());
        let s_rev = s.clone().rev();
        s.zip(s_rev).all(|(a, b)| a.eq_ignore_ascii_case(&b))
    }
}
