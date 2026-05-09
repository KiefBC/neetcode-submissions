use std::cmp;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut start = 0usize;
        let mut end = heights.len().saturating_sub(1);
        let mut maximum = 0;

        while start < end {
            let curr_max = (end - start) as i32 * cmp::min(heights[start], heights[end]);
            if curr_max > maximum {
                maximum = curr_max;
            }

            if heights[start] < heights[end] {
                start += 1;
            } else {
                end -= 1;
            }
        }

        maximum
    }
}
