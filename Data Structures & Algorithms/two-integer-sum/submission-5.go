func twoSum(nums []int, target int) []int {
	seen := make(map[int]int)
	for i, n := range nums {
		compliment := target - n
		if j, exists := seen[compliment]; exists {
			return []int{j, i}
		}
		seen[n] = i
	}

	return nil
}