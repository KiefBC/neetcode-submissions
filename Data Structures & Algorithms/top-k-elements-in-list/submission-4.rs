impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::with_capacity(k as usize + 1);

        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }

        for (n, c) in freq {
            heap.push(Reverse((c, n)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        heap.into_iter().map(|Reverse((_, n))| n).collect()
    }
}