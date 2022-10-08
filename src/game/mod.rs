pub struct IsoPath {
    board: [Tile;37],
}

impl IsoPath {
    pub fn new() -> Self {
        let mut board = [Tile::Normal;37];

        for i in 0..4 {
            board[i] = Tile::High(true);
            board[36-i] = Tile::Low(true);
        }

        IsoPath { board }
    }

    pub fn print_board(&self) {
        let mut ind = 0;
        for i in 0..7 {
            let w = (4+i).min(10-i);
            for _j in w..7 {
                print!("   ");
            }
            for j in 0..w {
                print!("|{}|", self.board[ind + j].repr());
            }
            ind += w;
            print!("\n");
        }
    }
}

pub enum Move {
    MoveMove(([usize;4],[usize;4])),
    MoveCap(([usize;4],[usize;2])),
    CapMove(([usize;2],[usize;4])),
}

impl Move {
    pub fn move_from_string(s : String) {
        
    }
}

#[derive(Clone, Copy)]
enum Tile {
    High(bool),
    Normal,
    Low(bool),
}

impl Tile {
    pub fn repr(&self) -> String {
        match self {
            Self::High(b) => format!("+{}+",if *b {"OO"} else {"++"}),
            Self::Normal => "====".to_string(),
            Self::Low(b) => format!("-{}-",if *b {"XX"} else {"--"}),
        }
    } 
}

