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
            let x = tile % MAP_SIZE;
            let y = tile / MAP_SIZE;
            map.set(Pos::new(x, y), true);
        }

        Piece::new(map, &offsets)
    }

    pub fn all() -> Vec<Name> {
        vec![Name::O, Name::S, Name::Z, Name::J, Name::L, Name::T, Name::I]
    }
}

const MAP_SIZE: usize = 4;
type Grid = [[bool; MAP_SIZE]; MAP_SIZE];

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    pub fn new(x: usize, y: usize) -> Self {
        Pos { x: x, y: y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Map {
    grid: Grid,
}

impl Map {
    pub fn new() -> Self {
        Map { grid: [[false; MAP_SIZE]; MAP_SIZE] }
    }

    pub fn in_bounds(&self, pos: Pos) -> bool {
        pos.x < MAP_SIZE && pos.y < MAP_SIZE
    }
    
    pub fn get(&self, pos: Pos) -> bool {
        self.in_bounds(pos) && self.grid[pos.y][pos.x]
    }

    pub fn set(&mut self, pos: Pos, value: bool) {
        if self.in_bounds(pos) {
            self.grid[pos.y][pos.x] = value;
        }
    }
        
    pub fn ccw(&self) -> Self {
        let mut new_map = Map::new();
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let pos = Pos::new(x, y);
                let pos0 = Pos::new(MAP_SIZE - 1 - y, x);
                new_map.set(pos, self.get(pos0));
            }
        }

        new_map
    }

    pub fn trans(&self, offset: [isize; 2]) -> Self {
        let mut new_map = Map::new();
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let x0 = x as isize - offset[0];
                let y0 = y as isize - offset[1];
                let value = 0 <= x0 && 0 <= y0 &&
                    self.get(Pos::new(x0 as usize, y0 as usize));

                new_map.set(Pos::new(x, y), value);
            }
        }

        new_map
    }
    
    pub fn draw(&self) -> String {
        let mut str = String::new();
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let tile = if self.get(Pos::new(x, y)) { "[]" } else { "--" };
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
    
    pub fn rot(&mut self, ccw: bool) {
        let len = self.maps.len();
        let step = if ccw { 1 } else { len - 1 };
        self.rot = (self.rot + step) % len;
    }

    pub fn draw(&self) -> String {
        self.get_map().draw()
    }
}
