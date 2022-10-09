pub struct IsoPath {
    board: [Tile;37],
    turn: Player,
}

impl IsoPath {
    pub fn new() -> Self {
        let mut board = [Tile::Normal;37];

        for i in 0..4 {
            board[i] = Tile::High(true);
            board[36-i] = Tile::Low(true);
        }

        IsoPath { board , turn: Player::Top}
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

    pub fn make_move(&mut self, movs: [Move;2]) -> Result<String,String> {
        let (m1,m2) = match movs {
            [Move::PieceMove(p1,p2), Move::TileMove(t1,t2)] => {
                (Move::PieceMove(p1, p2),Move::TileMove(t1, t2))
            },
            [ Move::TileMove(t1,t2),Move::PieceMove(p1,p2)] => {
                (Move::TileMove(t1, t2),Move::PieceMove(p1, p2))
            },
            [ Move::Capture(c),Move::PieceMove(p1,p2)] => {
                (Move::Capture(c),Move::PieceMove(p1, p2))
            },
            [Move::PieceMove(p1,p2),Move::Capture(c)] => {
                (Move::PieceMove(p1, p2), Move::Capture(c))
            },
            [Move::TileMove(t1,t2),Move::Capture(c)] => {
                (Move::TileMove(t1,t2),Move::Capture(c))
            },
            [Move::Capture(c),Move::TileMove(t1,t2)] => {
                (Move::Capture(c),Move::TileMove(t1,t2))
            },
            _ => return Err("This move-combination is not allowed".to_string()),
        };
        if self.move_legal(m1) {
            self.execute_move(m1);
            if self.move_legal(m2) {
                self.execute_move(m2);
            } else {
                self.undo_move(m1);
                return Err("The second move was illegal".to_string());
            }
        } else {
            return Err("The first move was illegal".to_string());
        }
        self.turn = self.turn.switch();
        Ok(String::new())
    }

    fn execute_move(&mut self, mov: Move) {
        match mov {
            Move::TileMove(m1,m2) => {
                self.board[m1] = self.board[m1].reduce();
                self.board[m2] = self.board[m2].increase();
            },
            Move::PieceMove(m1,m2) => {
                self.board[m1] = self.board[m1].switch();
                self.board[m2] = self.board[m2].switch();
            },
            Move::Capture(c) => {
                self.board[c] = self.board[c].switch();
            },
        }
    }

    fn undo_move(&mut self, mov: Move) {
        match mov {
            Move::TileMove(m1,m2) => {
                self.board[m1] = self.board[m1].increase();
                self.board[m2] = self.board[m2].reduce();
            },
            Move::PieceMove(m1,m2) => {
                self.board[m1] = self.board[m1].switch();
                self.board[m2] = self.board[m2].switch();
            },
            Move::Capture(c) => {
                self.board[c] = self.board[c].switch();
            },
        }
    }

    fn move_legal(&self, mov: Move) -> bool {
        match mov {
            Move::TileMove(m1, m2) => {
                let t1 = self.board[m1];
                let t2 = self.board[m2];
                let mut legal = true;
                match t1 {
                    Tile::Low(_) => legal = false,
                    Tile::High(true) => legal = false,
                    _ => (),
                }
                match t2 {
                    Tile::High(_) => legal = false,
                    Tile::Low(true) => legal = false,
                    _ => (),
                }

                // You cannot move tiles in the home row
                if self.turn == Player::Top {
                    if m1 <= 3 || m2 <= 3 {
                        legal = false;
                    }
                } else {
                    if m1 >= 33 || m2 >= 33 {
                        legal = false;
                    }
                }

                legal
            },
            Move::PieceMove(m1,m2 ) => {
                let t1 = self.board[m1];
                let t2 = self.board[m2];

                if self.turn == Player::Top {
                    IsoPath::get_adjacent(m1).iter().any(|m| *m == m2) && t1 == Tile::High(true) && t2 == Tile::High(false)
                } else {
                    IsoPath::get_adjacent(m1).iter().any(|m| *m == m2) && t1 == Tile::Low(true) && t2 == Tile::Low(false)
                }
            },
            Move::Capture(c) => {
                let t = self.board[c];
                if self.turn == Player::Top {
                    t == Tile::Low(true) && IsoPath::get_adjacent(c).iter().filter(|i| self.board[**i] == Tile::High(true)).count() >= 2
                }else {
                    t == Tile::High(true) && IsoPath::get_adjacent(c).iter().filter(|i| self.board[**i] == Tile::Low(true)).count() >= 2
                }
            },
        }
    }

    fn get_adjacent(pos: usize) -> Vec<usize>{
        let mut res = Vec::new();
        let mut col = 0;
        let mut i = 3;
        let mut w = 5;
        while i < pos {
            i += w;
            col += 1;
            w = (5+col).min(9-col);
        }
        if pos < i {
            res.push(pos + 1);
        }
        if pos as i32 + w as i32 - i as i32 - 1 > 0 {
            res.push(pos - 1);
        }
        let mut tl = 40;
        let mut tr = 40;
        let bl;
        let br;
        if w < 7 {
            if pos > w {
                tl = pos - w;
                tr = pos - w + 1;
            }
            bl = pos + w;
            br = pos + w + 1; 
        } else if w > 7 {
            if pos > w {
                tl = pos - w;
                tr = pos - w + 1;
            }
            bl = pos + w;
            br = pos + w - 1; 
        } else {
            if pos > w {
                tl = pos - w;
                tr = pos - w + 1;
            }
            bl = pos + w;
            br = pos + w - 1; 
        }

        if tl < 37 {
            res.push(tl);
            res.push(tr);
        }
        if br < 37 {
            res.push(bl);
            res.push(br);
        }

        match pos {
            0 => res.push(3),
            3 => res.push(0),
            15 => res.push(21),
            21 => res.push(15),
            33 => res.push(36),
            36 => res.push(33),
            _ => ()
        }

        res
    }

}

#[derive(PartialEq, Eq,Clone, Copy)]
enum Player {
    Top,
    Bottom,
}

impl Player {
    fn switch(&self) -> Self{
        match self {
            Player::Top => Player::Bottom,
            Player::Bottom => Player::Top,
        }
    } 
}

#[derive(Clone, Copy)]
pub enum Move {
    TileMove(usize,usize),
    PieceMove(usize,usize),
    Capture(usize)
}

impl Move {
    pub fn move_from_string(s : String) -> Result<[Move;2],String> {
        let mut v: Vec<&str> = s.split(' ').collect();
        if v.len() < 2 {
            return Err("Each turn consists of two moves".to_string());
        }
        let mut res = [Move::PieceMove(0, 0);2];
        for i in 0..2 {
            if v[i].starts_with("t") {
                v[i] = v[i].trim_matches('t');
                let m: Vec<&str> = v[i].split(&['-', ':', '@',';'][..]).collect();
                if m.len() < 2 {
                    return Err("A tile move must contain two coordinates".to_string());
                }
                let v1 = m[0].parse::<usize>();
                let v2 = m[1].parse::<usize>();
                let v1 = match v1 {
                    Ok(num) => num,
                    Err(_) => return Err("There was an error parsing a move".to_string()),
                } - 1;
                let v2 = match v2 {
                    Ok(num) => num,
                    Err(_) => return Err("There was an error parsing a move".to_string()),
                } - 1;
                if v1 >= 37 || v2 >= 37 {
                    return Err("The move was not on th board anymore".to_string());
                }
                res[i] = Move::TileMove(v1,v2);
            } else if v[i].starts_with("p") {
                v[i] = v[i].trim_matches('p');
                let m: Vec<&str> = v[i].split(&['-', ':', '@',';'][..]).collect();
                if m.len() < 2 {
                    return Err("A piece move must contain two coordinates".to_string());
                }
                let v1 = m[0].parse::<usize>();
                let v2 = m[1].parse::<usize>();
                let v1 = match v1 {
                    Ok(num) => num,
                    Err(_) => return Err("There was an error parsing a move".to_string()),
                } - 1;
                let v2 = match v2 {
                    Ok(num) => num,
                    Err(_) => return Err("There was an error parsing a move".to_string()),
                } - 1;
                if v1 >= 37 || v2 >= 37 {
                    return Err("The move was off the board".to_string());
                }
                res[i] = Move::PieceMove(v1,v2);
            } else if v[i].starts_with("c") {
                v[i] = v[i].trim_matches('c');
                let v = v[i].parse::<usize>();
                let v = match v {
                    Ok(num) => num,
                    Err(_) => return Err("There was an error parsing a move".to_string()),
                } - 1;
                if v >= 37 {
                    return Err("The move was not on th board anymore".to_string());
                }
                res[i] = Move::Capture(v);
            } else {
                return Err("No move was selected".to_string());
            }
        }
        Ok(res)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    High(bool),
    Normal,
    Low(bool),
}

impl Tile {
    fn repr(&self) -> String {
        match self {
            Self::High(b) => format!("+{}+",if *b {"OO"} else {"++"}),
            Self::Normal => "====".to_string(),
            Self::Low(b) => format!("-{}-",if *b {"XX"} else {"--"}),
        }
    } 

    fn reduce(&self) -> Self {
        match self {
            Self::High(b) => if !*b {
                Self::Normal
            } else {
                *self
            }
            Self::Normal => {
                Self::Low(false)
            },
            _ => *self,
        }
    }

    fn increase(&self) -> Self {
        match self {
            Self::Low(b) => if !*b {
                Self::Normal
            } else {
                *self
            }
            Self::Normal => {
                Self::High(false)
            },
            _ => *self,
        }
    }

    fn switch(&self) -> Self {
        match self {
            Self::High(b) => Self::High(!*b),
            Self::Low(b) => Self::Low(!*b),
            _ =>*self,
        }
    }
    fn occupied(&self) -> bool {
        match self {
            Self::High(b) => *b,
            Self::Low(b) => *b,
            Self::Normal => false,
        }
    }
}

