//! Contains traits related to dealing with "a 2-D plane of booleans"
//! (which is the conceptual model of a "bitstream" in this package)

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
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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

/// Trait that should be implemented on "the struct containing the bitstream's actual data".
pub trait BitArray {
    fn get(&self, c: Coordinate) -> bool;
    fn set(&mut self, c: Coordinate, val: bool);
}
