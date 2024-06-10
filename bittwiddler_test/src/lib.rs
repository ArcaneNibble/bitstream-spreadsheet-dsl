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

pub trait FieldAccessor {
    type Output;

    fn get(&self, bitstream: &impl BitArray) -> Self::Output;
}

impl TestBitstream {
    pub fn get_field<A: FieldAccessor>(&self, accessor: &A) -> A::Output {
        accessor.get(self)
    }
}

pub struct TilePropertyOneAccessor {
    x: u8,
    y: u8,
}
impl FieldAccessor for TilePropertyOneAccessor {
    type Output = u8;

    fn get(&self, bitstream: &impl BitArray) -> Self::Output {
        let mut ret = 0;

        if bitstream.get(self.x as usize * 4 + 0, self.y as usize * 4 + 0) {
            ret |= 1 << 0;
        }
        if bitstream.get(self.x as usize * 4 + 1, self.y as usize * 4 + 0) {
            ret |= 1 << 1;
        }
        if bitstream.get(self.x as usize * 4 + 2, self.y as usize * 4 + 0) {
            ret |= 1 << 2;
        }
        if bitstream.get(self.x as usize * 4 + 3, self.y as usize * 4 + 0) {
            ret |= 1 << 3;
        }
        if bitstream.get(self.x as usize * 4 + 0, self.y as usize * 4 + 1) {
            ret |= 1 << 4;
        }
        if bitstream.get(self.x as usize * 4 + 1, self.y as usize * 4 + 1) {
            ret |= 1 << 5;
        }
        if bitstream.get(self.x as usize * 4 + 2, self.y as usize * 4 + 1) {
            ret |= 1 << 6;
        }
        if bitstream.get(self.x as usize * 4 + 3, self.y as usize * 4 + 1) {
            ret |= 1 << 7;
        }

        ret
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
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
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

        assert_eq!(
            bitstream.get_field(&TilePropertyOneAccessor { x: 0, y: 0 }),
            1
        );
        assert_eq!(
            bitstream.get_field(&TilePropertyOneAccessor { x: 1, y: 1 }),
            2
        );
        assert_eq!(
            bitstream.get_field(&TilePropertyOneAccessor { x: 2, y: 1 }),
            3
        );
    }
}
