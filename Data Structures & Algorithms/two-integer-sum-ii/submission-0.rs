impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0usize;
        let mut right = numbers.len().saturating_sub(1);

        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum == target && left != right {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }

            if sum < target {
                left += 1;
            } else {
                right = right.saturating_sub(1);
            }
        }
        vec![]
    }
}
