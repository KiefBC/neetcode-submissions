impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs {
            let mut key = [0u8; 26];
            for b in s.bytes() {
                key[(b - b'a') as usize] += 1;
            }
            map.entry(key).or_default().push(s);
        }

        map.into_values().collect()
    }
}