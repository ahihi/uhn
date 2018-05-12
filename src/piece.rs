use ::pos::Pos;

#[derive(Debug)]
pub enum Name { O, S, Z, J, L, T, I }

impl Name {
    pub fn to_piece(&self) -> Piece {
        let mut map = Map::new();
        let (tiles, offsets): ([usize; 4], Vec<[isize; 2]>) = match self {
            &Name::O => (
                [1, 2, 5, 6],
                vec![]
            ),
            &Name::S => (
                [5, 6, 8, 9],
                vec![[-1, -1]]
            ),
            &Name::Z => (
                [4, 5, 9, 10],
                vec![[0, -1]]
            ),
            &Name::J => (
                [4, 5, 6, 10],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &Name::L => (
                [4, 5, 6, 8],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &Name::T => (
                [4, 5, 6, 9],
                vec![[0, -1], [0, 0], [-1, -1]]
            ),
            &Name::I => (
                [4, 5, 6, 7],
                vec![[1, 0]]
            )
        };

        for &tile in tiles.iter() {
            let x = tile % PIECE_SIZE;
            let y = tile / PIECE_SIZE;
            map.set(Pos::new(x as isize, y as isize), true);
        }

        Piece::new(map, &offsets)
    }

    pub fn all() -> Vec<Name> {
        vec![Name::O, Name::S, Name::Z, Name::J, Name::L, Name::T, Name::I]
    }
}

pub const PIECE_SIZE: usize = 4;
type Grid = [[bool; PIECE_SIZE]; PIECE_SIZE];

#[derive(Clone, Copy, Debug)]
pub struct Map {
    grid: Grid,
}

impl Map {
    pub fn new() -> Self {
        Map { grid: [[false; PIECE_SIZE]; PIECE_SIZE] }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        0 <= pos.x && pos.x < PIECE_SIZE as isize && 0 <= pos.y && pos.y < PIECE_SIZE as isize
    }
    
    pub fn get(&self, pos: Pos) -> bool {
        self.in_bounds(pos) && self.grid[pos.y as usize][pos.x as usize]
    }

    pub fn set(&mut self, pos: Pos, value: bool) {
        if self.in_bounds(pos) {
            self.grid[pos.y as usize][pos.x as usize] = value;
        }
    }
        
    pub fn ccw(&self) -> Self {
        let mut new_map = Map::new();
        for y in 0..PIECE_SIZE as isize {
            for x in 0..PIECE_SIZE as isize {
                let pos = Pos::new(x, y);
                let pos0 = Pos::new(PIECE_SIZE as isize - 1 - y, x);
                new_map.set(pos, self.get(pos0));
            }
        }

        new_map
    }

    pub fn trans(&self, offset: [isize; 2]) -> Self {
        let mut new_map = Map::new();
        for y in 0..PIECE_SIZE as isize {
            for x in 0..PIECE_SIZE as isize {
                let pos = Pos::new(x, y);
                let pos0 = Pos::new(x - offset[0], y - offset[1]);
                new_map.set(pos, self.get(pos0));
            }
        }

        new_map
    }
    
    pub fn draw(&self) -> String {
        let mut str = String::new();
        for y in 0..PIECE_SIZE as isize {
            for x in 0..PIECE_SIZE as isize {
                let tile = if self.get(Pos::new(x, y)) { "[ ]" } else { " . " };
                str.push_str(tile);
            }
            str.push('\n');
        }

        str
    }
}

#[derive(Debug)]
pub struct Piece {
    maps: Vec<Map>,
    rot: usize
}

impl Piece {
    pub fn new(map0: Map, offsets: &[[isize; 2]]) -> Self {
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
    
    pub fn get_map(&self) -> &Map {
        &self.maps[self.rot]
    }

    pub fn get(&self, pos: Pos) -> bool {
        self.get_map().get(pos)
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
