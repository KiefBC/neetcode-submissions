impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut set: HashSet<i32> = HashSet::new();
        let mut count = 0;

        for &n in &nums {
            if !set.insert(n) {
                continue; // if duplicate, skip
            }

            let mut current_count = 1;
            let mut left = n - 1;
            let mut right = n + 1;

            while set.contains(&left) {
                current_count += 1;
                left -= 1;
            }

            while set.contains(&right) {
                current_count += 1;
                right += 1;
            }

            count = count.max(current_count);
        }

        count
    }
}