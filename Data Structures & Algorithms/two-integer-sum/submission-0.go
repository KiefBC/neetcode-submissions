func twoSum(nums []int, target int) []int {
	result := make(map[int]int)

	for i, num := range nums {
		diff := target - num
		if idx, exists := result[num]; exists {
			return []int{idx, i}
		}

		result[diff] = i
	}

	return []int{}
}