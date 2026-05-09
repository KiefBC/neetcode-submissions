impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut left = 0usize;
        let mut right = height.len().saturating_sub(1);
        let mut left_m = 0;
        let mut right_m = 0;
        let mut water = 0;

        while left < right {
            if height[left] < height[right] {
                left_m = left_m.max(height[left]);
                water += left_m - height[left];
                left += 1;
            } else {
                right_m = right_m.max(height[right]);
                water += right_m - height[right];
                right -= 1;
            }
        }

        water
    }
}
