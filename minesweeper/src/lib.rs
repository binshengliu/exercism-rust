pub fn detect(board: &[Vec<char>], row: usize, col: usize) -> usize {
    vec![
        (row.checked_sub(1), col.checked_sub(1)), // top left
        (row.checked_sub(1), Some(col)),          // top
        (row.checked_sub(1), col.checked_add(1)), // top right
        (Some(row), col.checked_sub(1)),          // left
        (Some(row), col.checked_add(1)),          // right
        (row.checked_add(1), col.checked_sub(1)), // bottom left
        (row.checked_add(1), Some(col)),          // bottom
        (row.checked_add(1), col.checked_add(1)), // bottom right
    ].into_iter()
        // filter out invalid indexes
        .filter(|&(row_idx, col_idx)| row_idx.is_some() && col_idx.is_some())
        .filter(|&(row_idx, col_idx)| {
            row_idx.unwrap() < board.len() && col_idx.unwrap() < board[0].len()
        })
        // turn all valid indexes into raw format
        .map(|(row_idx, col_idx)| (row_idx.unwrap(), col_idx.unwrap()))
        // count '*'
        .filter(|&(row_idx, col_idx)| board[row_idx][col_idx] == '*')
        .count()
}

fn to_char(n: usize) -> char {
    if n == 0 {
        ' '
    } else {
        std::char::from_digit(n as u32, 10).unwrap()
    }
}

pub fn annotate(board: &[&str]) -> Vec<String> {
    // Easier for indexing
    let board_vec: Vec<Vec<char>> =
        board.iter().map(|row| row.chars().collect()).collect();

    board
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, elem)| match elem {
                    '*' => '*',
                    _ => to_char(detect(&board_vec, row_idx, col_idx)),
                })
                .collect()
        })
        .collect()
}
