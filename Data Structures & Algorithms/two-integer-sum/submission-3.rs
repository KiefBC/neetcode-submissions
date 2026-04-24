impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<&i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            let compliment: i32 = target - n;
            if let Some(&j) = map.get(&compliment) {
                return vec![j as i32, i as i32];
            }
            map.insert(n, i);
        }

        vec![]
    }
}