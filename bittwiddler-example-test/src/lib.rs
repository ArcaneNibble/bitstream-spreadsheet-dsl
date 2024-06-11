use std::{
    borrow::Cow,
    mem::{self, MaybeUninit},
};

use bittwiddler_core::prelude::*;
use itertools::Itertools;

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

pub trait FieldAccessor: ScriptingThingWithArgs {
    #[allow(private_bounds)]
    type BoolArray: MustBeABoolArrayConstGenericsWorkaround;
    type Output: PropertyLeafWithStringConv<Self::BoolArray, Self>
    where
        Self: Sized;

    fn get_bit_pos(&self, biti: usize) -> (usize, usize);

    fn get(&self, bitstream: &(impl BitArray + ?Sized)) -> Self::Output
    where
        Self: Sized,
    {
        let mut bits: <Self::BoolArray as MustBeABoolArrayConstGenericsWorkaround>::MaybeUninitTy =
            unsafe { MaybeUninit::uninit().assume_init() };
        for biti in 0..Self::BoolArray::NBITS {
            let (x, y) = self.get_bit_pos(biti);
            bits.as_mut()[biti].write(bitstream.get(x, y));
        }
        let bits = unsafe { mem::transmute_copy::<_, Self::BoolArray>(&bits) };
        Self::Output::from_bits(&bits)
    }
    fn set(&self, bitstream: &mut (impl BitArray + ?Sized), val: Self::Output)
    where
        Self: Sized,
    {
        let bits = val.to_bits();
        for biti in 0..Self::BoolArray::NBITS {
            let (x, y) = self.get_bit_pos(biti);
            bitstream.set((x, y).into(), bits.as_ref()[biti]);
        }
    }

    fn get_as_string(&self, bitstream: &(impl BitArray + ?Sized)) -> Cow<'static, str>
    where
        Self: Sized,
    {
        let val = self.get(bitstream);
        val.to_string(self)
    }
    fn set_from_string(&self, bitstream: &mut (impl BitArray + ?Sized), val: &str)
    where
        Self: Sized,
    {
        let val = Self::Output::from_string(val, self);
        self.set(bitstream, val);
    }
}

impl TestBitstream {
    pub fn get_field<A: FieldAccessor>(&self, accessor: &A) -> A::Output {
        accessor.get(self)
    }
    pub fn set_field<A: FieldAccessor>(&mut self, accessor: &A, val: A::Output) {
        accessor.set(self, val);
    }

    pub fn get_as_string<A: FieldAccessor>(&self, accessor: &A) -> Cow<'static, str> {
        accessor.get_as_string(self)
    }
    pub fn set_from_string<A: FieldAccessor>(&mut self, accessor: &A, val: &str) {
        accessor.set_from_string(self, val);
    }
}

pub trait ScriptingArgSink {
    fn add_arg(&mut self, arg: &str, val: &str);
}

pub trait ScriptingThingWithArgs {
    fn _scripting_dump_my_args(&self, dump: &mut dyn ScriptingArgSink);
}

pub trait FieldAccessorDyn: ScriptingThingWithArgs {
    fn _scripting_get(&self, bitstream: &dyn BitArray) -> Cow<'static, str>;
    fn _scripting_set(&self, bitstream: &mut dyn BitArray, val: &str);
}
impl<A: FieldAccessor> FieldAccessorDyn for Box<A> {
    fn _scripting_get(&self, bitstream: &dyn BitArray) -> Cow<'static, str> {
        self.get_as_string(bitstream)
    }

    fn _scripting_set(&self, bitstream: &mut dyn BitArray, val: &str) {
        self.set_from_string(bitstream, val);
    }
}
impl<A: FieldAccessor> ScriptingThingWithArgs for Box<A> {
    fn _scripting_dump_my_args(&self, dump: &mut dyn ScriptingArgSink) {
        A::_scripting_dump_my_args(self, dump)
    }
}

pub trait AccessorScriptingMetadata: ScriptingThingWithArgs {
    fn _scripting_fields(&self) -> &'static [&'static str];
    fn _scripting_sublevels(&self) -> &'static [&'static str];

    fn _scripting_construct_field(&self, idx: usize, params: &[&str]) -> Box<dyn FieldAccessorDyn>;
    fn _scripting_construct_all_fields<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn FieldAccessorDyn>> + 's>;

    fn _scripting_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Box<dyn AccessorScriptingMetadata>;
    fn _scripting_construct_all_sublevels<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn AccessorScriptingMetadata>> + 's>;
}

impl AccessorScriptingMetadata for TestBitstream {
    fn _scripting_fields(&self) -> &'static [&'static str] {
        &[]
    }

    fn _scripting_sublevels(&self) -> &'static [&'static str] {
        &["tile"]
    }

    fn _scripting_construct_field(
        &self,
        _idx: usize,
        _params: &[&str],
    ) -> Box<dyn FieldAccessorDyn> {
        unreachable!()
    }

    fn _scripting_construct_all_fields<'s>(
        &'s self,
        _idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn FieldAccessorDyn>> + 's> {
        unreachable!()
    }

    fn _scripting_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Box<dyn AccessorScriptingMetadata> {
        match idx {
            0 => Box::new(Tile::tile(
                params[0].parse().unwrap(),
                params[1].parse().unwrap(),
            )),
            _ => unreachable!(),
        }
    }
    fn _scripting_construct_all_sublevels<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn AccessorScriptingMetadata>> + 's> {
        match idx {
            0 => Box::new(
                (0..4)
                    .cartesian_product(0..4)
                    .map(|(y, x)| Box::new(Tile::tile(x, y)) as Box<dyn AccessorScriptingMetadata>),
            ),
            _ => unreachable!(),
        }
    }
}
impl AccessorScriptingMetadata for Tile {
    fn _scripting_fields(&self) -> &'static [&'static str] {
        &[
            "property_one",
            "property_two",
            "property_three",
            "property_four",
        ]
    }

    fn _scripting_sublevels(&self) -> &'static [&'static str] {
        &[]
    }

    fn _scripting_construct_field(&self, idx: usize, params: &[&str]) -> Box<dyn FieldAccessorDyn> {
        match idx {
            0 => Box::new(Box::new(self.property_one())),
            1 => Box::new(Box::new(self.property_two(params[0].parse().unwrap()))),
            2 => Box::new(Box::new(self.property_three())),
            3 => Box::new(Box::new(self.property_four())),
            _ => unreachable!(),
        }
    }
    fn _scripting_construct_all_fields<'s>(
        &'s self,
        idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn FieldAccessorDyn>> + 's> {
        match idx {
            0 => Box::new(
                [Box::new(Box::new(self.property_one())) as Box<dyn FieldAccessorDyn>].into_iter(),
            ),
            1 => Box::new(
                (0..4)
                    .map(|n| Box::new(Box::new(self.property_two(n))) as Box<dyn FieldAccessorDyn>),
            ),
            2 => Box::new(
                [Box::new(Box::new(self.property_three())) as Box<dyn FieldAccessorDyn>]
                    .into_iter(),
            ),
            3 => Box::new(
                [Box::new(Box::new(self.property_four())) as Box<dyn FieldAccessorDyn>].into_iter(),
            ),
            _ => unreachable!(),
        }
    }

    fn _scripting_descend_sublevel(
        &self,
        _idx: usize,
        _params: &[&str],
    ) -> Box<dyn AccessorScriptingMetadata> {
        unreachable!()
    }
    fn _scripting_construct_all_sublevels<'s>(
        &'s self,
        _idx: usize,
    ) -> Box<dyn Iterator<Item = Box<dyn AccessorScriptingMetadata>> + 's> {
        unreachable!()
    }
}
impl ScriptingThingWithArgs for Tile {
    fn _scripting_dump_my_args(&self, dump: &mut dyn ScriptingArgSink) {
        dump.add_arg("x", &ToString::to_string(&self.x));
        dump.add_arg("y", &ToString::to_string(&self.y));
    }
}
impl ScriptingThingWithArgs for TestBitstream {
    fn _scripting_dump_my_args(&self, _dump: &mut dyn ScriptingArgSink) {}
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
impl FieldAccessor for TilePropertyOneAccessor {
    type BoolArray = [bool; 8];
    type Output = u8;

    fn get_bit_pos(&self, biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4 + (biti % 4);
        let y = self.tile.y as usize * 4 + (biti / 4);
        (x, y)
    }
}
impl ScriptingThingWithArgs for TilePropertyOneAccessor {
    fn _scripting_dump_my_args(&self, _dump: &mut dyn ScriptingArgSink) {}
}

pub struct TilePropertyTwoAccessor {
    tile: Tile,
    n: u8,
}
impl FieldAccessor for TilePropertyTwoAccessor {
    type BoolArray = [bool; 1];
    type Output = bool;

    fn get_bit_pos(&self, _biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4 + self.n as usize;
        let y = self.tile.y as usize * 4 + 2;
        (x, y)
    }
}
impl ScriptingThingWithArgs for TilePropertyTwoAccessor {
    fn _scripting_dump_my_args(&self, dump: &mut dyn ScriptingArgSink) {
        dump.add_arg("n", &ToString::to_string(&self.n));
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
impl FieldAccessor for TilePropertyThreeAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, _biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4;
        let y = self.tile.y as usize * 4 + 3;
        (x, y)
    }
}
impl ScriptingThingWithArgs for TilePropertyThreeAccessor {
    fn _scripting_dump_my_args(&self, _dump: &mut dyn ScriptingArgSink) {}
}
pub struct TilePropertyFourAccessor {
    tile: Tile,
}
impl FieldAccessor for TilePropertyFourAccessor {
    type BoolArray = [bool; 1];
    type Output = CustomBool;

    fn get_bit_pos(&self, _biti: usize) -> (usize, usize) {
        let x = self.tile.x as usize * 4 + 1;
        let y = self.tile.y as usize * 4 + 3;
        (x, y)
    }
}
impl ScriptingThingWithArgs for TilePropertyFourAccessor {
    fn _scripting_dump_my_args(&self, _dump: &mut dyn ScriptingArgSink) {}
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
            _0, _1, _0, _0,     _1, _0, _0, _0,     _0, _0, _0, _0,     _0, _0, _0, _0,

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

        assert_eq!(
            bitstream.get_as_string(&Tile::tile(0, 0).property_one()),
            "00000001"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(1, 1).property_one()),
            "00000010"
        );
        assert_eq!(
            bitstream.get_as_string(&Tile::tile(2, 1).property_one()),
            "00000011"
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
        bitstream.set_field(&Tile::tile(0, 0).property_one(), 0xa5);
        bitstream.set_from_string(&Tile::tile(1, 1).property_one(), "01011010");

        let bit_str = bitstream.to_string();
        print!("{}", bit_str);
    }

    struct SimpleStringSink<'a> {
        first: bool,
        s: &'a mut String,
    }
    impl<'a> ScriptingArgSink for SimpleStringSink<'a> {
        fn add_arg(&mut self, arg: &str, val: &str) {
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
    fn test_scripting() {
        let bitstream = TestBitstream { bits: [false; 256] };

        fn recurse(bitstream: &impl BitArray, level: &dyn AccessorScriptingMetadata, prefix: &str) {
            for (sublevel_idx, sublevel_name) in level._scripting_sublevels().iter().enumerate() {
                for sublevel_obj in level._scripting_construct_all_sublevels(sublevel_idx) {
                    let mut sublevel_full_name = prefix.to_string();
                    sublevel_full_name.push_str(sublevel_name);
                    sublevel_full_name.push('[');
                    let mut x = SimpleStringSink {
                        first: true,
                        s: &mut sublevel_full_name,
                    };
                    sublevel_obj._scripting_dump_my_args(&mut x);
                    sublevel_full_name.push_str("].");
                    recurse(bitstream, &*sublevel_obj, &sublevel_full_name);
                }
            }

            for (field_idx, field_name) in level._scripting_fields().iter().enumerate() {
                for field_obj in level._scripting_construct_all_fields(field_idx) {
                    let mut field_full_name = prefix.to_string();
                    field_full_name.push_str(field_name);
                    field_full_name.push('[');
                    let mut x = SimpleStringSink {
                        first: true,
                        s: &mut field_full_name,
                    };
                    field_obj._scripting_dump_my_args(&mut x);
                    field_full_name.push(']');
                    let result_str = field_obj._scripting_get(bitstream);
                    println!("{} = {}", field_full_name, result_str);
                }
            }
        }

        let meta_root: &dyn AccessorScriptingMetadata = &bitstream;
        recurse(&bitstream, meta_root, "");
    }
}
