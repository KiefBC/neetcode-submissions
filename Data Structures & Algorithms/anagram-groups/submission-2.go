func groupAnagrams(strs []string) [][]string {
	anagramGroups := make(map[[26]byte][]string)

	for _, str := range strs {
		var charCount [26]byte
		for _, char := range str {
			charCount[char-'a']++
		}
		anagramGroups[charCount] = append(anagramGroups[charCount], str)
	}

	res := make([][]string, 0, len(anagramGroups))
	for _, group := range anagramGroups {
		res = append(res, group)
	}
	return res
}