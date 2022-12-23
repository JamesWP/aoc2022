use std::fs::DirBuilder;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Ord, PartialOrd)]
pub struct Pos2d {
    pub x: i32,
    pub y: i32,
}

pub enum Direction {
    Up,
    Left,
    Right,
    Down,
    None,
}

impl Pos2d {
    pub fn up(self) -> Pos2d {
        Pos2d {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(self) -> Pos2d {
        Pos2d {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(self) -> Pos2d {
        Pos2d {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(self) -> Pos2d {
        Pos2d {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn move_in_direction(self, d: Direction) -> Pos2d {
        match d {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
            Direction::Down => self.down(),
            Direction::None => self,
        }
    }

    pub fn cmp_no_diagonal(&self, other: &Pos2d) -> Direction {
        if self.x < other.x {
            assert_eq!(self.y, other.y);
            Direction::Right
        } else if self.x > other.x {
            assert_eq!(self.y, other.y);
            Direction::Left
        } else {
            if self.y < other.y {
                assert_eq!(self.x, other.x);
                Direction::Down
            } else if self.y > other.y {
                assert_eq!(self.x, other.x);
                Direction::Up
            } else {
                Direction::None
            }
        }
    }
}

impl<'a> From<&'a str> for Pos2d {
    fn from(coords: &str) -> Self {
        let mut parts = coords.split(",");
        let x = parts.next().unwrap().trim().parse().unwrap();
        let y = parts.next().unwrap().trim().parse().unwrap();

        Pos2d { x, y }
    }
}

impl Default for Pos2d {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
