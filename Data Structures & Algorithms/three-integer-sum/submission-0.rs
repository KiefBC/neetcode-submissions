impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                for k in (j + 1)..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut triplet = vec![nums[i], nums[j], nums[k]];
                        triplet.sort_unstable();
                        if !result.contains(&triplet) {
                            result.push(triplet);
                        }
                    }
                }
            }
        }

        result
    }
}
