use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BoardPiece {
    Red,
    Black,
    None,
}

const ROW_COL_SIZE: usize = 5;
static CURRENT_PLAYER: Mutex<BoardPiece> = Mutex::new(BoardPiece::Red);
static PIECES_DROPPED: [AtomicI32; 2] = [AtomicI32::new(0), AtomicI32::new(0)];

pub fn make_blank_board() -> [[BoardPiece; ROW_COL_SIZE]; ROW_COL_SIZE] {
    [[BoardPiece::None; ROW_COL_SIZE]; ROW_COL_SIZE]
}
pub struct GameState {
    pub board: [[BoardPiece; ROW_COL_SIZE]; ROW_COL_SIZE],
}

impl GameState {    

    pub fn print_board(&self) {
        let mut label: String;
        for row in 0..ROW_COL_SIZE {
            for col in 0..ROW_COL_SIZE {
                if self.board[row][col] == BoardPiece::None {
                    label = "-".to_string();
                }
                else if self.board[row][col] == BoardPiece::Red {
                    label = "R".to_string();
                }
                else {
                    label = "B".to_string();
                }

                print!("{}", label);
            }
            println!()
        }
        println!()
    }

    pub fn handle_click(&mut self, row: usize, col: usize) {
        println!("Clicked at ({col}, {row})");

        if self.board[row][col] != BoardPiece::None {
            return;
        }

        // Get Current Player from Mutex for reading
        {
            let current_player = CURRENT_PLAYER.lock().unwrap();

            if self.get_pieces_dropped(*current_player) >= 4 {
                return;
            }

            self.board[row][col] = *current_player;
        }

        self.next_turn();
    }

    fn next_turn(&mut self) {
        // Get Current Player from Mutex for modifying
        let mut current_player = CURRENT_PLAYER.lock().unwrap(); 
        self.add_piece_count(*current_player);

        *current_player = match *current_player {
            BoardPiece::Red => BoardPiece::Black,
            _ => BoardPiece::Red,
        }
    }

    fn index_of_piece(&self, piece: BoardPiece) -> usize {
        match piece {
            BoardPiece::Red => 0,
            _ => 1
        }
    }

    fn get_pieces_dropped(&self, piece: BoardPiece) -> i32 {
        PIECES_DROPPED[self.index_of_piece(piece)].load(Ordering::Relaxed)
    }

    fn add_piece_count(&self, piece: BoardPiece) -> i32 {
        PIECES_DROPPED[self.index_of_piece(piece)].fetch_add(1, Ordering::Relaxed)
    }

}