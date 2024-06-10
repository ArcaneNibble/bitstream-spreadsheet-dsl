pub struct TestBitstream {
    pub bits: [bool; 256],
}

pub trait BitArray {
    fn get(&self, x: usize, y: usize) -> bool;
    fn set(&mut self, x: usize, y: usize, val: bool);
}
impl BitArray for TestBitstream {
    fn get(&self, x: usize, y: usize) -> bool {
        self.bits[y * 16 + x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        self.bits[y * 16 + x] = val;
    }
}

impl ToString for TestBitstream {
    fn to_string(&self) -> String {
        let mut ret = String::with_capacity(17 * 16);
        for y in 0..16 {
            for x in 0..16 {
                if self.get(x, y) {
                    ret.push('1');
                } else {
                    ret.push('0');
                }
            }
            ret.push('\n');
        }
        ret
    }
}

pub trait FieldAccessor {
    type Output;
    const NBITS: usize;

    fn get_bit_pos(&self, biti: usize) -> (usize, usize);

    fn get(&self, bitstream: &impl BitArray) -> Self::Output;
    fn set(&self, bitstream: &mut impl BitArray, val: Self::Output);
}

impl TestBitstream {
    pub fn get_field<A: FieldAccessor>(&self, accessor: &A) -> A::Output {
        accessor.get(self)
    }
    pub fn set_field<A: FieldAccessor>(&mut self, accessor: &A, val: A::Output) {
        accessor.set(self, val);
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    x: u8,
    y: u8,
}
impl Tile {
    pub fn tile(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn property_one(&self) -> TilePropertyOneAccessor {
        TilePropertyOneAccessor { tile: self.clone() }
    }
    pub fn property_two(&self, n: u8) -> TilePropertyTwoAccessor {
        TilePropertyTwoAccessor {
            tile: self.clone(),
            n,
        }
    }
}

pub struct TilePropertyOneAccessor {
    tile: Tile,
}
impl FieldAccessor for TilePropertyOneAccessor {
    type Output = u8;

    const NBITS: usize = 8;
    fn get_bit_pos(&self, biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4 + (biti % 4);
        let y = self.tile.y as usize * 4 + (biti / 4);
        (x, y)
    }

    fn get(&self, bitstream: &impl BitArray) -> Self::Output {
        let mut ret = 0;

        for biti in 0..Self::NBITS {
            let (x, y) = self.get_bit_pos(biti);
            if bitstream.get(x, y) {
                ret |= 1 << biti;
            }
        }

        ret
    }

    fn set(&self, bitstream: &mut impl BitArray, val: Self::Output) {
        for biti in 0..Self::NBITS {
            let (x, y) = self.get_bit_pos(biti);
            bitstream.set(x, y, val & 1 << biti != 0);
        }
    }
}

pub struct TilePropertyTwoAccessor {
    tile: Tile,
    n: u8,
}
impl FieldAccessor for TilePropertyTwoAccessor {
    type Output = bool;

    const NBITS: usize = 1;
    fn get_bit_pos(&self, _biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4 + self.n as usize;
        let y = self.tile.y as usize * 4 + 2;
        (x, y)
    }

    fn get(&self, bitstream: &impl BitArray) -> Self::Output {
        let (x, y) = self.get_bit_pos(0);
        bitstream.get(x, y)
    }

    fn set(&self, bitstream: &mut impl BitArray, val: Self::Output) {
        let (x, y) = self.get_bit_pos(0);
        bitstream.set(x, y, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const _1: bool = true;
    const _0: bool = false;

    #[test]
    fn test_get() {
        #[rustfmt::skip]
        let bitstream = TestBitstream { bits: [
            _1, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _1, _1, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,

            _0, _0, _0, _0,     _0, _1, _0, _0,     _1, _1, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,

            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,

            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
        ] };

        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_one()), 1);
        assert_eq!(bitstream.get_field(&Tile::tile(1, 1).property_one()), 2);
        assert_eq!(bitstream.get_field(&Tile::tile(2, 1).property_one()), 3);

        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(0)), _0);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(1)), _1);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(2)), _1);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(3)), _0);
    }

    #[test]
    fn test_set() {
        let mut bitstream = TestBitstream { bits: [false; 256] };
        bitstream.set_field(&Tile::tile(0, 0).property_one(), 0xa5);

        let bit_str = bitstream.to_string();
        print!("{}", bit_str);
    }
}
