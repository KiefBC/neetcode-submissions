impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in &strs {
            result.push_str(&format!("{}#{}", &s.len().to_string(), s));
        }

        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            let mut j = s[i..].find('#').unwrap() + i;
            let len: usize = s[i..j]
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse length from '{}'", &s[i..j]));

            i = j + 1;
            j = i + len;
            result.push(s[i..j].to_string());
            i = j;
        }

        result
    }
}