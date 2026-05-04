impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_map: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut col_map: HashMap<usize, HashSet<char>> = HashMap::new();
        let mut box_map: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] != '.' {
                    if !row_map.entry(row).or_default().insert(board[row][col])
                        || !col_map.entry(col).or_default().insert(board[row][col])
                        || !box_map
                            .entry((row / 3, col / 3))
                            .or_default()
                            .insert(board[row][col])
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
}
