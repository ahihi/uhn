#[derive(Debug)]
pub enum PieceName { O, S, Z, J, L, T, I }

impl PieceName {
    pub fn to_piece(&self) -> Piece {
        let mut map = PieceMap::new();
        let (bits, offsets): ([u16; 4], Vec<[i16; 2]>) = match self {
            &PieceName::O => (
                [1, 2, 5, 6],
                vec![]
            ),
            &PieceName::S => (
                [5, 6, 8, 9],
                vec![[-1, -1]]
            ),
            &PieceName::Z => (
                [4, 5, 9, 10],
                vec![[0, -1]]
            ),
            &PieceName::J => (
                [4, 5, 6, 10],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &PieceName::L => (
                [4, 5, 6, 8],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &PieceName::T => (
                [4, 5, 6, 9],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &PieceName::I => (
                [4, 5, 6, 7],
                vec![[1, 0]]
            )
        };

        for &bit in bits.iter() {
            map.set_bit(bit, true);
        }

        Piece::new(map, &offsets)
    }

    pub fn all() -> Vec<PieceName> {
        vec![PieceName::O, PieceName::S, PieceName::Z, PieceName::J, PieceName::L, PieceName::T, PieceName::I]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PieceMap {
    map: u16,
}

impl PieceMap {
    pub fn new() -> Self {
        PieceMap { map: 0 }
    }

    pub fn get_bit(&self, bit: u16) -> bool {
         self.map & (1 << bit) != 0
    }

    pub fn pos_bit(&self, x: u16, y: u16) -> u16 {
        4 * y + x
    }
    
    pub fn set_bit(&mut self, bit: u16, on: bool) {
        let mask: u16 = 1u16 << bit;
        if on {
            self.map |= mask;
        } else {
            self.map &= !mask;
        }
    }

    pub fn set_map(&mut self, map: u16) {
        self.map = map;
    }
    
    pub fn ccw(&self) -> Self {
        let mut new_map = PieceMap::new();
        for bit in 0..16 {
            let bit0 = (15 - bit) / 4 + (bit % 4) * 4;
            let value = self.get_bit(bit0);
            new_map.set_bit(bit, value);
        }

        new_map
    }

    pub fn trans(&self, offset: [i16; 2]) -> Self {
        let mut new_map = PieceMap::new();
        for y in 0..4 {
            for x in 0..4 {
                let x0 = x - offset[0];
                let y0 = y - offset[1];
                let value = 0 <= x0 && x0 < 4 && 0 <= y0 && y0 < 4 &&
                    self.get_bit(self.pos_bit(x0 as u16, y0 as u16));

                new_map.set_bit(self.pos_bit(x as u16, y as u16), value);
            }
        }

        new_map
    }
    
    pub fn draw(&self) -> String {
        let mut str = String::new();
        for bit in 0..16 {
            let tile = if self.get_bit(bit) { "[]" } else { "--" };
            str.push_str(tile);
            if bit % 4 == 3 {
                str.push('\n');
            }
        }

        str
    }
}

#[derive(Debug)]
pub struct Piece {
    maps: Vec<PieceMap>,
    rot: usize
}

impl Piece {
    pub fn new(map0: PieceMap, offsets: &[[i16; 2]]) -> Self {
        let mut map = map0;
        let mut maps = vec![map];
        for &offset in offsets {
            map = map.ccw().trans(offset);
            maps.push(map);
        }

        Piece { maps: maps, rot: 0 }
    }

    pub fn len(&self) -> usize {
        self.maps.len()
    }
    
    pub fn get_map(&self) -> &PieceMap {
        &self.maps[self.rot]
    }
    
    pub fn rot(&mut self, ccw: bool) {
        let len = self.maps.len();
        let step = if ccw { 1 } else { len - 1 };
        self.rot = (self.rot + step) % len;
    }

    pub fn draw(&self) -> String {
        self.get_map().draw()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
