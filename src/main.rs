/*
Enumerate the board as follows:
0 = empty
1...infinity = player ID

e.g. 
If player 1 fills a square, the value for that square is set to 1.
There can be N number of players, up to a max of std::u32:MAX
 */
fn init_board(dim: usize) -> Vec<Vec<u32>> {
    let mut board = Vec::with_capacity(dim);

    for _i in 0..dim {
        // initialize board to false = not filled
        let mut row = Vec::with_capacity(dim);
        for _j in 0..dim {
            row.push(0);
        }
        
        board.push(row);
    }

    board
}

fn check_win(board: &Vec<Vec<u32>>) -> u32 {
    0
}

fn take_user_input(board: &Vec<Vec<u32>>) -> (u32, usize, usize) {
}

fn main() {
    // initialize board
    let mut board = init_board(3);

    let mut winner = 0;
    loop {
        winner = check_win(&board);
        if winner > 0 {
            break;
        }

        let (player_id, i, j) = take_user_input(&board);
        board[i][j] = player_id;
    }

    println!("Congratulations, the winner is player {}!", winner);
}