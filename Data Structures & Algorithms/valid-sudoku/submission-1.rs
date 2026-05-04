impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // let mut row_map: HashMap<usize, HashSet<char>> = HashMap::new();
        // let mut col_map: HashMap<usize, HashSet<char>> = HashMap::new();
        // let mut box_map: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        let mut row_vec: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut col_vec: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut box_vec: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for row in 0..9 {
            for col in 0..9 {
                // if board[row][col] != '.' {
                //     if row_map.entry(row).or_default().insert(board[row][col])
                //         || col_map.entry(col).or_default().insert(board[row][col])
                //         || box_map
                //             .entry((row / 3, col / 3))
                //             .or_default()
                //             .insert(board[row][col])
                //     {
                //         return false;
                //     }
                // }

                let cell = board[row][col];
                if cell == '.' {
                    continue;
                }
                let box_index = (row / 3) * 3 + col / 3;
                if !row_vec[row].insert(cell)
                    || !col_vec[col].insert(cell)
                    || !box_vec[box_index].insert(cell)
                {
                    return false;
                }
            }
        }

        true
    }
}