impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let set: HashSet<i32> = nums.into_iter().collect();
        let mut count = 0;

        for &n in &set {
            if !set.contains(&(n - 1)) {
                let mut curr = n;
                let mut length = 1;
                while set.contains(&(curr + 1)) {
                    length += 1;
                    curr += 1;
                }

                count = count.max(length);
            }
        }

        count
    }
}