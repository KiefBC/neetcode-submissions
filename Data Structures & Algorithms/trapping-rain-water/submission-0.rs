impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut prefix_arr: Vec<i32> = vec![0; height.len()];
        prefix_arr[0] = height[0];
        let mut suffix_arr: Vec<i32> = vec![0; height.len()];
        suffix_arr[height.len() - 1] = height[height.len() - 1];
        let mut total_amount = 0;

        for i in 1..height.len() {
            let curr_max = prefix_arr[i - 1].max(height[i]);
            prefix_arr[i] = curr_max;
        }

        for j in (0..height.len() - 1).rev() {
            let curr_max = suffix_arr[j + 1].max(height[j]);
            suffix_arr[j] = curr_max;
        }

        for k in 0..height.len() {
            let water = prefix_arr[k].min(suffix_arr[k]);
            total_amount += water - height[k];
        }

        total_amount
    }
}
