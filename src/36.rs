use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set: HashSet<char> = HashSet::new();
        for i in 0..9 {
            set.clear();
            for j in 0..9 {
                if board[i][j] != '.' && !set.insert(board[i][j]) {
                    return false;
                }
            }

            set.clear();
            for j in 0..9 {
                if board[j][i] != '.' && !set.insert(board[j][i]) {
                    return false;
                }
            }

            set.clear();
            let row_index = 3 * (i / 3);
            let col_index = 3 * (i % 3);
            for j in 0..9 {
                let row = row_index + j / 3;
                let col = col_index + j % 3;
                if board[row][col] != '.' && !set.insert(board[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}
