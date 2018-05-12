use ::piece::{PIECE_SIZE, Piece};
use ::pos::Pos;

const WELL_HEIGHT: usize = 20;
const WELL_WIDTH: usize = 10;

#[derive(Debug)]
pub struct Well {
    grid: [[bool; WELL_WIDTH]; WELL_HEIGHT]
}

impl Well {
    pub fn new() -> Self {
        Well { grid: [[false; WELL_WIDTH]; WELL_HEIGHT] }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        0 <= pos.x && pos.x < WELL_WIDTH as isize && 0 <= pos.y && pos.y < WELL_HEIGHT as isize
    }
    
    pub fn get(&self, pos: Pos) -> bool {
        !self.in_bounds(pos) || self.grid[pos.y as usize][pos.x as usize]
    }

    pub fn set(&mut self, pos: Pos, value: bool) {
        if self.in_bounds(pos) {
            self.grid[pos.y as usize][pos.x as usize] = value;
        }
    }

    pub fn get_spawn_pos(&self, piece: &Piece) -> Pos {
        let mut empty_top_rows = 0;
        'outer: for y in 0..PIECE_SIZE as isize {
            for x in 0..PIECE_SIZE as isize {
                if piece.get(Pos::new(x, y)) {
                    break 'outer;
                }
            }
            empty_top_rows += 1;
        }

        Pos::new(3, -empty_top_rows)
    }
    
    pub fn fits(&self, pos: Pos, piece: &Piece) -> bool {
        for dy in 0..PIECE_SIZE as isize {
            for dx in 0..PIECE_SIZE as isize {
                let well_pos = Pos::new(pos.x + dx, pos.y + dy);                
                let piece_pos = Pos::new(dx, dy);
                
                if self.get(well_pos) && piece.get(piece_pos) {
                    return false;
                }
            }
        }

        true
    }

    pub fn imprint(&mut self, pos: Pos, piece: &Piece) {
        for dy in 0..PIECE_SIZE as isize {
            for dx in 0..PIECE_SIZE as isize {
                let well_pos = Pos::new(pos.x + dx, pos.y + dy);                
                let piece_pos = Pos::new(dx, dy);
                
                if piece.get(piece_pos) {
                    self.set(well_pos, true);
                }
            }
        }
    }
    
    pub fn draw(&self) -> String {
        let mut str = String::new();

        for y in 0..WELL_HEIGHT as isize {
            for x in 0..WELL_WIDTH as isize {
                let tile = if self.get(Pos::new(x, y)) { "[ ]" } else { " . " };
                str.push_str(tile);
            }

            str.push('\n');
        }
        
        str
    }
}
