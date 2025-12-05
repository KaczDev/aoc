pub type Grid<T> = Vec<Vec<T>>;

/// (r, c)
pub type Point = (usize, usize);

/// Checks if the position (r,c) + (dr,dc) would lead to out of range in the grid
pub fn will_be_oob<T>(grid: &Grid<T>, (r, c): Point, direction: Direction) -> bool {
    let (dr, dc) = direction.resolve();
    let r = r.checked_add_signed(dr);
    let c = c.checked_add_signed(dc);
    r.is_none() || c.is_none() || r.unwrap() >= grid.len() || c.unwrap() >= grid[0].len()
}

/// Unchecked access to grid, panics if out of range
pub fn get_element<T>(grid: &Grid<T>, (r, c): Point) -> &T {
    &grid[r][c]
}

/// Returns a tuple of found needle (row, column)
/// panics otherwise
pub fn find_element<T: std::cmp::PartialEq + std::fmt::Debug>(grid: &Grid<T>, needle: T) -> Point {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == needle {
                return (r, c);
            }
        }
    }
    panic!("COULDNT FIND ELEMENT {:?}", needle);
}

/// Prints the grid in between newlines
pub fn print_grid<T: std::fmt::Debug>(grid: &Grid<T>) {
    println!();
    for r in grid {
        for c in r {
            print!("{:?}", c);
        }
        println!();
    }
    println!();
}

pub fn add_points((cur_row, cur_col): Point, (dr, dc): Point) -> Point {
    (cur_row + dr, cur_col + dc)
}

#[derive(Copy, Debug, Clone, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    RighUp,
    RightDown,
    LeftUp,
    LeftDown,
}

impl Direction {
    pub fn all_straight() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }
    pub fn all_diagonals() -> [Direction; 8] {
        [
            Direction::RighUp,
            Direction::RightDown,
            Direction::LeftUp,
            Direction::LeftDown,
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }
    pub fn resolve(self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::RighUp => (-1, 1),
            Direction::RightDown => (1, 1),
            Direction::LeftUp => (-1, -1),
            Direction::LeftDown => (1, -1),
        }
    }
    pub fn rotate_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            _ => todo!(),
        }
    }
    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => todo!(),
        }
    }

    /// Add direction to Point, doesn't check for underflows. Will panic if underflow happens
    pub fn move_point(self, (cur_row, cur_col): Point) -> Point {
        let (dr, dc) = self.resolve();
        let cur_row = cur_row
            .checked_add_signed(dr)
            .expect(&format!("Couldn't add {} to {}", dr, cur_row));
        let cur_col = cur_col
            .checked_add_signed(dc)
            .expect(&format!("Couldn't add {} to {}", dc, cur_col));
        (cur_row, cur_col)
    }
}
