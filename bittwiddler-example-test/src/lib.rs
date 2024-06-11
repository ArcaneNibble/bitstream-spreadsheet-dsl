#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use bittwiddler_core::prelude::*;
use bittwiddler_macros::*;
#[cfg(feature = "alloc")]
use itertools::Itertools;

include!(concat!(env!("OUT_DIR"), "/bitproperty-out.rs"));
include!(concat!(env!("OUT_DIR"), "/tiles-out.rs"));

pub struct TestBitstream {
    pub bits: [bool; 256],
}

impl BitArray for TestBitstream {
    fn get(&self, c: Coordinate) -> bool {
        self.bits[c.y * 16 + c.x]
    }

    fn set(&mut self, c: Coordinate, val: bool) {
        self.bits[c.y * 16 + c.x] = val;
    }
}

impl ToString for TestBitstream {
    fn to_string(&self) -> String {
        let mut ret = String::with_capacity(17 * 16);
        for y in 0..16 {
            for x in 0..16 {
                if self.get((x, y).into()) {
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

impl TestBitstream {
    pub fn get_field<A: PropertyAccessor>(&self, accessor: &A) -> A::Output {
        accessor.get(self)
    }
    pub fn set_field<A: PropertyAccessor>(&mut self, accessor: &A, val: A::Output) {
        accessor.set(self, val);
    }

    #[cfg(feature = "alloc")]
    pub fn get_as_string<A: PropertyAccessor + PropertyAccessorWithStringConv>(
        &self,
        accessor: &A,
    ) -> Cow<'static, str>
    where
        A::Output: PropertyLeafWithStringConv<A::BoolArray, A>,
    {
        accessor.get_as_string(self)
    }
    #[cfg(feature = "alloc")]
    pub fn set_from_string<A: PropertyAccessor + PropertyAccessorWithStringConv>(
        &mut self,
        accessor: &A,
        val: &str,
    ) where
        A::Output: PropertyLeafWithStringConv<A::BoolArray, A>,
    {
        accessor.set_from_string(self, val).unwrap();
    }
}

#[cfg(feature = "alloc")]
impl HumanLevelThatHasState for TestBitstream {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
}

#[cfg(feature = "alloc")]
impl TestBitstreamAutomagicRequiredFunctions for TestBitstream {
    fn _automagic_construct_all_tile(&self) -> impl Iterator<Item = Tile> {
        (0..4)
            .cartesian_product(0..4)
            .map(|(y, x)| Self::tile(x, y))
    }
}

#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl TestBitstream {
    pub fn tile(x: u8, y: u8) -> Tile {
        Tile { x, y }
    }
    pub fn dummy_sublevel() -> DummySublevel {
        DummySublevel
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
#[derive(Clone, Copy)]
pub struct Tile {
    x: u8,
    y: u8,
}

#[cfg(feature = "alloc")]
impl TileAutomagicRequiredFunctions for Tile {
    fn _automagic_construct_all_property_two(
        &self,
    ) -> impl Iterator<Item = TilePropertyTwoAccessor> {
        (0..4).map(|n| self.property_two(n))
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct DummySublevel;
#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl DummySublevel {
    #[bittwiddler::property]
    pub fn dummy_field() -> DummySublevelField {
        DummySublevelField
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct DummySublevelField;
impl PropertyAccessor for DummySublevelField {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> Coordinate {
        Coordinate::new(15, 15)
    }
}

#[bittwiddler_properties(alloc_feature_gate = "alloc")]
impl Tile {
    #[bittwiddler::property]
    pub fn property_one(&self) -> TilePropertyOneAccessor {
        TilePropertyOneAccessor { tile: self.clone() }
    }
    #[bittwiddler::property]
    pub fn property_two(&self, n: u8) -> TilePropertyTwoAccessor {
        TilePropertyTwoAccessor {
            tile: self.clone(),
            n,
        }
    }
    #[bittwiddler::property]
    pub fn property_three(&self) -> TilePropertyThreeAccessor {
        TilePropertyThreeAccessor { tile: self.clone() }
    }
    #[bittwiddler::property]
    pub fn property_four(&self) -> TilePropertyFourAccessor {
        TilePropertyFourAccessor { tile: self.clone() }
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct TilePropertyOneAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyOneAccessor {
    type BoolArray = [bool; 4];
    type Output = Property1;

    fn get_bit_pos(&self, biti: usize) -> Coordinate {
        test_tile::PROPERTY_ONE[biti]
            + Coordinate::new(
                self.tile.x as usize * test_tile::W,
                self.tile.y as usize * test_tile::H,
            )
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct TilePropertyTwoAccessor {
    tile: Tile,
    n: u8,
}
impl PropertyAccessor for TilePropertyTwoAccessor {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, biti: usize) -> Coordinate {
        test_tile::PROPERTY_TWO[self.n as usize][biti]
            + Coordinate::new(
                self.tile.x as usize * test_tile::W,
                self.tile.y as usize * test_tile::H,
            )
    }
}

pub struct CustomBool(bool);
impl PropertyLeaf<[bool; 1]> for CustomBool {
    fn from_bits(bits: &[bool; 1]) -> Self {
        CustomBool(bits[0])
    }

    fn to_bits(&self) -> [bool; 1] {
        [self.0]
    }
}
#[cfg(feature = "alloc")]
impl PropertyLeafWithStringConv<[bool; 1], TilePropertyThreeAccessor> for CustomBool {
    fn to_string(&self, accessor: &TilePropertyThreeAccessor) -> Cow<'static, str> {
        if !self.0 {
            "nonono".into()
        } else {
            format!("({}, {})", accessor.tile.x, accessor.tile.y).into()
        }
    }

    fn from_string(s: &str, _accessor: &TilePropertyThreeAccessor) -> Result<Self, ()> {
        if s == "nonono" {
            Ok(Self(false))
        } else {
            Ok(Self(true))
        }
    }
}
#[cfg(feature = "alloc")]
impl PropertyLeafWithStringConv<[bool; 1], TilePropertyFourAccessor> for CustomBool {
    fn to_string(&self, accessor: &TilePropertyFourAccessor) -> Cow<'static, str> {
        if !self.0 {
            "lalala".into()
        } else {
            format!("[{}, {}]", accessor.tile.x, accessor.tile.y).into()
        }
    }

    fn from_string(s: &str, _accessor: &TilePropertyFourAccessor) -> Result<Self, ()> {
        if s == "lalala" {
            Ok(Self(false))
        } else {
            Ok(Self(true))
        }
    }
}
#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct TilePropertyThreeAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyThreeAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, biti: usize) -> Coordinate {
        test_tile::PROPERTY_THREE[biti]
            + Coordinate::new(
                self.tile.x as usize * test_tile::W,
                self.tile.y as usize * test_tile::H,
            )
    }
}

#[bittwiddler_hierarchy_level(alloc_feature_gate = "alloc")]
pub struct TilePropertyFourAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyFourAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, biti: usize) -> Coordinate {
        test_tile::PROPERTY_FOUR[biti]
            + Coordinate::new(
                self.tile.x as usize * test_tile::W,
                self.tile.y as usize * test_tile::H,
            )
    }
}

#[cfg(all(test, feature = "alloc"))]
mod tests {
    use super::*;
    const _1: bool = true;
    const _0: bool = false;

    #[test]
    fn test_get() {
        #[rustfmt::skip]
        let bitstream = TestBitstream { bits: [
            _0, _0, _0, _1,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _1, _1, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,
            _0, _1, _0, _0,     _1, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,

            _0, _0, _0, _0,     _0, _0, _1, _0,     _0, _0, _1, _1,     _0, _0, _0, _0,
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
            bitstream.get_field(&TestBitstream::tile(0, 0).property_one()),
            Property1::ChoiceOne
        );
        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(1, 1).property_one()),
            Property1::ChoiceTwo
        );
        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(2, 1).property_one()),
            Property1::ChoiceThree
        );

        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(0, 0).property_two(0)),
            _0
        );
        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(0, 0).property_two(1)),
            _1
        );
        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(0, 0).property_two(2)),
            _1
        );
        assert_eq!(
            bitstream.get_field(&TestBitstream::tile(0, 0).property_two(3)),
            _0
        );

        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(0, 0).property_one()),
            "ChoiceOne"
        );
        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(1, 1).property_one()),
            "ChoiceTwo"
        );
        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(2, 1).property_one()),
            "ChoiceThree"
        );

        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(0, 0).property_three()),
            "nonono"
        );
        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(1, 0).property_three()),
            "(1, 0)"
        );

        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(0, 0).property_four()),
            "[0, 0]"
        );
        assert_eq!(
            bitstream.get_as_string(&TestBitstream::tile(1, 0).property_four()),
            "lalala"
        );
    }

    #[test]
    fn test_set() {
        let mut bitstream = TestBitstream { bits: [false; 256] };
        bitstream.set_field(
            &TestBitstream::tile(0, 0).property_one(),
            Property1::ChoiceWithX([_0, _1, _1, _0]),
        );
        bitstream.set_from_string(
            &TestBitstream::tile(1, 1).property_one(),
            "ChoiceWithX(0101)",
        );

        let bit_str = bitstream.to_string();
        print!("{}", bit_str);
    }

    #[test]
    fn test_human() {
        let bitstream = TestBitstream { bits: [false; 256] };
        let mut out = Vec::new();
        bittwiddler_textfile::write(&mut out, &bitstream).unwrap();

        print!("{}", std::str::from_utf8(&out).unwrap());
    }
}
