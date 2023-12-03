struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut counts = [0; 128];
        for ch in s.bytes() {
            counts[ch as usize] += 1;
        }

        for ch in t.bytes() {
            if counts[ch as usize] == 0 {
                return false;
            }
            counts[ch as usize] -= 1;
        }
        true
    }
}
