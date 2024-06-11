use std::borrow::Cow;

use bittwiddler_core::prelude::*;
use itertools::Itertools;

include!(concat!(env!("OUT_DIR"), "/bitproperty-out.rs"));

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

    pub fn get_as_string<A: PropertyAccessor + PropertyAccessorWithStringConv>(
        &self,
        accessor: &A,
    ) -> Cow<'static, str>
    where
        A::Output: PropertyLeafWithStringConv<A::BoolArray, A>,
    {
        accessor.get_as_string(self)
    }
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

impl HumanLevelDynamicAccessor for TestBitstream {
    fn _human_fields(&self) -> &'static [&'static str] {
        &[]
    }

    fn _human_sublevels(&self) -> &'static [&'static str] {
        &["tile"]
    }

    fn _human_construct_field(
        &self,
        _idx: usize,
        _params: &[&str],
    ) -> Box<dyn PropertyAccessorDyn> {
        unreachable!()
    }

    fn _human_construct_all_fields<'s>(
        &'s self,
        _idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn PropertyAccessorDyn>> + 's> {
        unreachable!()
    }

    fn _human_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Box<dyn HumanLevelDynamicAccessor> {
        match idx {
            0 => Box::new(Tile::tile(
                StatePiece::from_human_string(params[0]).unwrap(),
                params[1].parse().unwrap(),
            )),
            _ => unreachable!(),
        }
    }
    fn _human_construct_all_sublevels<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn HumanLevelDynamicAccessor>> + 's> {
        match idx {
            0 => Box::new(
                (0..4)
                    .cartesian_product(0..4)
                    .map(|(y, x)| Box::new(Tile::tile(x, y)) as Box<dyn HumanLevelDynamicAccessor>),
            ),
            _ => unreachable!(),
        }
    }
}
impl HumanLevelDynamicAccessor for Tile {
    fn _human_fields(&self) -> &'static [&'static str] {
        &[
            "property_one",
            "property_two",
            "property_three",
            "property_four",
        ]
    }

    fn _human_sublevels(&self) -> &'static [&'static str] {
        &[]
    }

    fn _human_construct_field(&self, idx: usize, params: &[&str]) -> Box<dyn PropertyAccessorDyn> {
        match idx {
            0 => Box::new(Box::new(self.property_one())),
            1 => Box::new(Box::new(self.property_two(params[0].parse().unwrap()))),
            2 => Box::new(Box::new(self.property_three())),
            3 => Box::new(Box::new(self.property_four())),
            _ => unreachable!(),
        }
    }
    fn _human_construct_all_fields<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn PropertyAccessorDyn>> + 's> {
        match idx {
            0 => Box::new(
                [Box::new(Box::new(self.property_one())) as Box<dyn PropertyAccessorDyn>]
                    .into_iter(),
            ),
            1 => {
                Box::new((0..4).map(|n| {
                    Box::new(Box::new(self.property_two(n))) as Box<dyn PropertyAccessorDyn>
                }))
            }
            2 => Box::new(
                [Box::new(Box::new(self.property_three())) as Box<dyn PropertyAccessorDyn>]
                    .into_iter(),
            ),
            3 => Box::new(
                [Box::new(Box::new(self.property_four())) as Box<dyn PropertyAccessorDyn>]
                    .into_iter(),
            ),
            _ => unreachable!(),
        }
    }

    fn _human_descend_sublevel(
        &self,
        _idx: usize,
        _params: &[&str],
    ) -> Box<dyn HumanLevelDynamicAccessor> {
        unreachable!()
    }
    fn _human_construct_all_sublevels<'s>(
        &'s self,
        _idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn HumanLevelDynamicAccessor>> + 's> {
        unreachable!()
    }
}
impl HumanLevelThatHasState for Tile {
    fn _human_dump_my_state(&self, dump: &mut dyn HumanSinkForStatePieces) {
        dump.add_state_piece("x", &self.x.to_human_string());
        dump.add_state_piece("y", &ToString::to_string(&self.y));
    }
}
impl HumanLevelThatHasState for TestBitstream {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
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
    pub fn property_three(&self) -> TilePropertyThreeAccessor {
        TilePropertyThreeAccessor { tile: self.clone() }
    }
    pub fn property_four(&self) -> TilePropertyFourAccessor {
        TilePropertyFourAccessor { tile: self.clone() }
    }
}

pub struct TilePropertyOneAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyOneAccessor {
    type BoolArray = [bool; 4];
    type Output = Property1;

    fn get_bit_pos(&self, biti: usize) -> Coordinate {
        let x = self.tile.x as usize * 4 + biti;
        let y = self.tile.y as usize * 4;
        (x, y).into()
    }
}
impl HumanLevelThatHasState for TilePropertyOneAccessor {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
}

pub struct TilePropertyTwoAccessor {
    tile: Tile,
    n: u8,
}
impl PropertyAccessor for TilePropertyTwoAccessor {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> Coordinate {
        let x = self.tile.x as usize * 4 + self.n as usize;
        let y = self.tile.y as usize * 4 + 2;
        (x, y).into()
    }
}
impl HumanLevelThatHasState for TilePropertyTwoAccessor {
    fn _human_dump_my_state(&self, dump: &mut dyn HumanSinkForStatePieces) {
        dump.add_state_piece("n", &ToString::to_string(&self.n));
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
pub struct TilePropertyThreeAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyThreeAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, _biti: usize) -> Coordinate {
        let x = self.tile.x as usize * 4;
        let y = self.tile.y as usize * 4 + 3;
        (x, y).into()
    }
}
impl HumanLevelThatHasState for TilePropertyThreeAccessor {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
}
pub struct TilePropertyFourAccessor {
    tile: Tile,
}
impl PropertyAccessor for TilePropertyFourAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, _biti: usize) -> Coordinate {
        let x = self.tile.x as usize * 4 + 1;
        let y = self.tile.y as usize * 4 + 3;
        (x, y).into()
    }
}
impl HumanLevelThatHasState for TilePropertyFourAccessor {
    fn _human_dump_my_state(&self, _dump: &mut dyn HumanSinkForStatePieces) {}
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
            bitstream.get_field(&Tile::tile(0, 0).property_one()),
            Property1::ChoiceOne
        );
        assert_eq!(
            bitstream.get_field(&Tile::tile(1, 1).property_one()),
            Property1::ChoiceTwo
        );
        assert_eq!(
            bitstream.get_field(&Tile::tile(2, 1).property_one()),
            Property1::ChoiceThree
        );

        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(0)), _0);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(1)), _1);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(2)), _1);
        assert_eq!(bitstream.get_field(&Tile::tile(0, 0).property_two(3)), _0);

        assert_eq!(
            bitstream.get_as_string(&Tile::tile(0, 0).property_one()),
            "ChoiceOne"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(1, 1).property_one()),
            "ChoiceTwo"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(2, 1).property_one()),
            "ChoiceThree"
        );

        assert_eq!(
            bitstream.get_as_string(&Tile::tile(0, 0).property_three()),
            "nonono"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(1, 0).property_three()),
            "(1, 0)"
        );

        assert_eq!(
            bitstream.get_as_string(&Tile::tile(0, 0).property_four()),
            "[0, 0]"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(1, 0).property_four()),
            "lalala"
        );
    }

    #[test]
    fn test_set() {
        let mut bitstream = TestBitstream { bits: [false; 256] };
        bitstream.set_field(
            &Tile::tile(0, 0).property_one(),
            Property1::ChoiceWithX([_0, _1, _1, _0]),
        );
        bitstream.set_from_string(&Tile::tile(1, 1).property_one(), "ChoiceWithX(0101)");

        let bit_str = bitstream.to_string();
        print!("{}", bit_str);
    }

    struct SimpleStringSink<'a> {
        first: bool,
        s: &'a mut String,
    }
    impl<'a> HumanSinkForStatePieces for SimpleStringSink<'a> {
        fn add_state_piece(&mut self, arg: &str, val: &str) {
            if !self.first {
                self.s.push_str(", ");
            }
            self.first = false;

            self.s.push_str(arg);
            self.s.push('=');
            self.s.push_str(val);
        }
    }

    #[test]
    fn test_human() {
        let bitstream = TestBitstream { bits: [false; 256] };

        fn recurse(bitstream: &impl BitArray, level: &dyn HumanLevelDynamicAccessor, prefix: &str) {
            for (sublevel_idx, sublevel_name) in level._human_sublevels().iter().enumerate() {
                for sublevel_obj in level._human_construct_all_sublevels(sublevel_idx) {
                    let mut sublevel_full_name = prefix.to_string();
                    sublevel_full_name.push_str(sublevel_name);
                    sublevel_full_name.push('[');
                    let mut x = SimpleStringSink {
                        first: true,
                        s: &mut sublevel_full_name,
                    };
                    sublevel_obj._human_dump_my_state(&mut x);
                    sublevel_full_name.push_str("].");
                    recurse(bitstream, &*sublevel_obj, &sublevel_full_name);
                }
            }

            for (field_idx, field_name) in level._human_fields().iter().enumerate() {
                for field_obj in level._human_construct_all_fields(field_idx) {
                    let mut field_full_name = prefix.to_string();
                    field_full_name.push_str(field_name);
                    field_full_name.push('[');
                    let mut x = SimpleStringSink {
                        first: true,
                        s: &mut field_full_name,
                    };
                    field_obj._human_dump_my_state(&mut x);
                    field_full_name.push(']');
                    let result_str = field_obj._human_string_get(bitstream);
                    println!("{} = {}", field_full_name, result_str);
                }
            }
        }

        let meta_root: &dyn HumanLevelDynamicAccessor = &bitstream;
        recurse(&bitstream, meta_root, "");
    }
}
