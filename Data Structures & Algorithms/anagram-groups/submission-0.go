func groupAnagrams(strs []string) [][]string {
	anagramMap := make(map[[26]byte][]string, len(strs))

	for _, str := range strs {
		freq := [26]byte{}

		for _, b := range []byte(str) {
			freq[b-'a']++
		}

		anagramMap[freq] = append(anagramMap[freq], str)
	}

	result := make([][]string, 0, len(anagramMap))
	for _, group := range anagramMap {
		result = append(result, group)
	}

	return result
}