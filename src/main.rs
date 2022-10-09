pub mod game;
use crate::game::IsoPath;
use crate::game::Move;
use std::io;

fn main() {
    let mut board = IsoPath::new();
    board.print_board();
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        println!("Input your move:");
        let ok = stdin.read_line(&mut line).is_ok();
        line = line.trim().to_string();
        if line.eq("stop") || !ok {
            break;
        }
        match Move::move_from_string(line) {
            Ok(movs) => {
                match board.make_move(movs) {
                    Ok(s) => {
                        board.print_board();
                        println!("{}",s);
                    },
                    Err(e) => {
                        board.print_board();
                        println!("{}",e);
                    },
                }
            },
            Err(e) => println!("{}",e),
        }

    }
}
