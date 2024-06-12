//! Contains traits related to dealing with "a 2-D plane of booleans"
//! (which is the conceptual model of a "bitstream" in this package)

use core::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// An `(x, y)` coordinate
///
/// This package uses the "graphics" coordinate convention
/// where +x is right and +y is down:
/// ```text
/// (0, 0) ----> +x
///  |
///  |
///  \/
///  +y
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}
impl Coordinate {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub const fn sub_x_add_y(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y + rhs.y,
        }
    }

    pub const fn add_x_sub_y(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}
impl From<Coordinate> for (usize, usize) {
    fn from(value: Coordinate) -> Self {
        (value.x, value.y)
    }
}
impl Display for Coordinate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Trait that should be implemented on "the struct containing the bitstream's actual data".
pub trait BitArray {
    fn get(&self, c: Coordinate) -> bool;
    fn set(&mut self, c: Coordinate, val: bool);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_ops() {
        let mut c1 = Coordinate::new(1, 2);
        let c2 = Coordinate::new(3, 4);

        assert_eq!(c1 + c2, Coordinate::new(4, 6));
        c1 += c2;
        assert_eq!(c1, Coordinate::new(4, 6));

        assert_eq!(c1 - c2, Coordinate::new(1, 2));
        c1 -= c2;
        assert_eq!(c1, Coordinate::new(1, 2));
    }
}
