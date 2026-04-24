impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::with_capacity(s.len());

        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }

        for c in t.chars() {
            match map.get_mut(&c) {
                Some(count) if *count > 0 => *count -= 1,
                _ => return false,
            }
        }

        true
    }
}