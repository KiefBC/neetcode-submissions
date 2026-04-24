impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = [0; 26];

        for (c1, c2) in s.chars().zip(t.chars()) {
            map[(c1 as usize) - ('a' as usize)] += 1;
            map[(c2 as usize) - ('a' as usize)] -= 1;
        }

        for count in map {
            if count != 0 {
                return false;
            }
        }

        true
    }
}