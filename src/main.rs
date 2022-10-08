use game::IsoPath;
pub mod game;

fn main() {
    let mut board = IsoPath::new();
    board.print_board();
}
