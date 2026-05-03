impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        let mut prefix = 1;
        for (i, &num) in nums.iter().enumerate() {
            res[i] = prefix;
            prefix *= num;
        }

        let mut suffix = 1;
        for (i, &num) in nums.iter().enumerate().rev() {
            res[i] *= suffix;
            suffix *= num;
        }

        res
    }
}
