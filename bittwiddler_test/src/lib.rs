use std::{
    borrow::Cow,
    mem::{self, MaybeUninit},
};

use itertools::Itertools;

pub struct TestBitstream {
    pub bits: [bool; 256],
}

trait MustBeABoolArrayConstGenericsWorkaround: AsRef<[bool]> {
    type MaybeUninitTy: AsMut<[MaybeUninit<bool>]>;
    const NBITS: usize;
}
impl<const N: usize> MustBeABoolArrayConstGenericsWorkaround for [bool; N] {
    type MaybeUninitTy = [MaybeUninit<bool>; N];
    const NBITS: usize = N;
}

#[allow(private_bounds)]
pub trait BitProperty<T: MustBeABoolArrayConstGenericsWorkaround> {
    fn from_bits(bits: &T) -> Self;
    fn to_bits(&self) -> T;
}
#[allow(private_bounds)]
pub trait BitPropertyWithStringConv<T: MustBeABoolArrayConstGenericsWorkaround, A: FieldAccessor>:
    BitProperty<T>
{
    fn to_string(&self, _accessor: &A) -> Cow<'static, str> {
        let bits = self.to_bits();
        let mut s = String::with_capacity(T::NBITS);
        for i in (0..T::NBITS).rev() {
            if bits.as_ref()[i] {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.into()
    }
    fn from_string(s: &str, _accessor: &A) -> Self
    where
        Self: Sized,
    {
        let mut bits: T::MaybeUninitTy = unsafe { MaybeUninit::uninit().assume_init() };
        for (i, c) in s.chars().rev().enumerate() {
            if c == '1' {
                bits.as_mut()[i].write(true);
            } else if c == '0' {
                bits.as_mut()[i].write(false);
            } else {
                unreachable!()
            }
        }
        let bits = unsafe { mem::transmute_copy::<_, T>(&bits) };
        Self::from_bits(&bits)
    }
}

impl BitProperty<[bool; 1]> for bool {
    fn from_bits(bits: &[bool; 1]) -> Self {
        bits[0]
    }

    fn to_bits(&self) -> [bool; 1] {
        [*self]
    }
}
impl<A: FieldAccessor> BitPropertyWithStringConv<[bool; 1], A> for bool {}

macro_rules! impl_bit_prop_for_int {
    ($nbits:expr, $int_ty:ty) => {
        impl BitProperty<[bool; $nbits]> for $int_ty {
            fn from_bits(bits: &[bool; $nbits]) -> Self {
                let mut ret = 0;
                for i in 0..$nbits {
                    if bits[i] {
                        ret |= 1 << i;
                    }
                }
                ret
            }

            fn to_bits(&self) -> [bool; $nbits] {
                let mut ret = [false; $nbits];
                for i in 0..$nbits {
                    if (self & (1 << i)) != 0 {
                        ret[i] = true;
                    }
                }
                ret
            }
        }
        impl<A: FieldAccessor> BitPropertyWithStringConv<[bool; $nbits], A> for $int_ty {}
    };
}

impl_bit_prop_for_int!(1, u8);
impl_bit_prop_for_int!(2, u8);
impl_bit_prop_for_int!(3, u8);
impl_bit_prop_for_int!(4, u8);
impl_bit_prop_for_int!(5, u8);
impl_bit_prop_for_int!(6, u8);
impl_bit_prop_for_int!(7, u8);
impl_bit_prop_for_int!(8, u8);

impl_bit_prop_for_int!(1, u16);
impl_bit_prop_for_int!(2, u16);
impl_bit_prop_for_int!(3, u16);
impl_bit_prop_for_int!(4, u16);
impl_bit_prop_for_int!(5, u16);
impl_bit_prop_for_int!(6, u16);
impl_bit_prop_for_int!(7, u16);
impl_bit_prop_for_int!(8, u16);
impl_bit_prop_for_int!(9, u16);
impl_bit_prop_for_int!(10, u16);
impl_bit_prop_for_int!(11, u16);
impl_bit_prop_for_int!(12, u16);
impl_bit_prop_for_int!(13, u16);
impl_bit_prop_for_int!(14, u16);
impl_bit_prop_for_int!(15, u16);
impl_bit_prop_for_int!(16, u16);

impl_bit_prop_for_int!(1, u32);
impl_bit_prop_for_int!(2, u32);
impl_bit_prop_for_int!(3, u32);
impl_bit_prop_for_int!(4, u32);
impl_bit_prop_for_int!(5, u32);
impl_bit_prop_for_int!(6, u32);
impl_bit_prop_for_int!(7, u32);
impl_bit_prop_for_int!(8, u32);
impl_bit_prop_for_int!(9, u32);
impl_bit_prop_for_int!(10, u32);
impl_bit_prop_for_int!(11, u32);
impl_bit_prop_for_int!(12, u32);
impl_bit_prop_for_int!(13, u32);
impl_bit_prop_for_int!(14, u32);
impl_bit_prop_for_int!(15, u32);
impl_bit_prop_for_int!(16, u32);
impl_bit_prop_for_int!(17, u32);
impl_bit_prop_for_int!(18, u32);
impl_bit_prop_for_int!(19, u32);
impl_bit_prop_for_int!(20, u32);
impl_bit_prop_for_int!(21, u32);
impl_bit_prop_for_int!(22, u32);
impl_bit_prop_for_int!(23, u32);
impl_bit_prop_for_int!(24, u32);
impl_bit_prop_for_int!(25, u32);
impl_bit_prop_for_int!(26, u32);
impl_bit_prop_for_int!(27, u32);
impl_bit_prop_for_int!(28, u32);
impl_bit_prop_for_int!(29, u32);
impl_bit_prop_for_int!(30, u32);
impl_bit_prop_for_int!(31, u32);
impl_bit_prop_for_int!(32, u32);

impl_bit_prop_for_int!(1, u64);
impl_bit_prop_for_int!(2, u64);
impl_bit_prop_for_int!(3, u64);
impl_bit_prop_for_int!(4, u64);
impl_bit_prop_for_int!(5, u64);
impl_bit_prop_for_int!(6, u64);
impl_bit_prop_for_int!(7, u64);
impl_bit_prop_for_int!(8, u64);
impl_bit_prop_for_int!(9, u64);
impl_bit_prop_for_int!(10, u64);
impl_bit_prop_for_int!(11, u64);
impl_bit_prop_for_int!(12, u64);
impl_bit_prop_for_int!(13, u64);
impl_bit_prop_for_int!(14, u64);
impl_bit_prop_for_int!(15, u64);
impl_bit_prop_for_int!(16, u64);
impl_bit_prop_for_int!(17, u64);
impl_bit_prop_for_int!(18, u64);
impl_bit_prop_for_int!(19, u64);
impl_bit_prop_for_int!(20, u64);
impl_bit_prop_for_int!(21, u64);
impl_bit_prop_for_int!(22, u64);
impl_bit_prop_for_int!(23, u64);
impl_bit_prop_for_int!(24, u64);
impl_bit_prop_for_int!(25, u64);
impl_bit_prop_for_int!(26, u64);
impl_bit_prop_for_int!(27, u64);
impl_bit_prop_for_int!(28, u64);
impl_bit_prop_for_int!(29, u64);
impl_bit_prop_for_int!(30, u64);
impl_bit_prop_for_int!(31, u64);
impl_bit_prop_for_int!(32, u64);
impl_bit_prop_for_int!(33, u64);
impl_bit_prop_for_int!(34, u64);
impl_bit_prop_for_int!(35, u64);
impl_bit_prop_for_int!(36, u64);
impl_bit_prop_for_int!(37, u64);
impl_bit_prop_for_int!(38, u64);
impl_bit_prop_for_int!(39, u64);
impl_bit_prop_for_int!(40, u64);
impl_bit_prop_for_int!(41, u64);
impl_bit_prop_for_int!(42, u64);
impl_bit_prop_for_int!(43, u64);
impl_bit_prop_for_int!(44, u64);
impl_bit_prop_for_int!(45, u64);
impl_bit_prop_for_int!(46, u64);
impl_bit_prop_for_int!(47, u64);
impl_bit_prop_for_int!(48, u64);
impl_bit_prop_for_int!(49, u64);
impl_bit_prop_for_int!(50, u64);
impl_bit_prop_for_int!(51, u64);
impl_bit_prop_for_int!(52, u64);
impl_bit_prop_for_int!(53, u64);
impl_bit_prop_for_int!(54, u64);
impl_bit_prop_for_int!(55, u64);
impl_bit_prop_for_int!(56, u64);
impl_bit_prop_for_int!(57, u64);
impl_bit_prop_for_int!(58, u64);
impl_bit_prop_for_int!(59, u64);
impl_bit_prop_for_int!(60, u64);
impl_bit_prop_for_int!(61, u64);
impl_bit_prop_for_int!(62, u64);
impl_bit_prop_for_int!(63, u64);
impl_bit_prop_for_int!(64, u64);

impl_bit_prop_for_int!(1, u128);
impl_bit_prop_for_int!(2, u128);
impl_bit_prop_for_int!(3, u128);
impl_bit_prop_for_int!(4, u128);
impl_bit_prop_for_int!(5, u128);
impl_bit_prop_for_int!(6, u128);
impl_bit_prop_for_int!(7, u128);
impl_bit_prop_for_int!(8, u128);
impl_bit_prop_for_int!(9, u128);
impl_bit_prop_for_int!(10, u128);
impl_bit_prop_for_int!(11, u128);
impl_bit_prop_for_int!(12, u128);
impl_bit_prop_for_int!(13, u128);
impl_bit_prop_for_int!(14, u128);
impl_bit_prop_for_int!(15, u128);
impl_bit_prop_for_int!(16, u128);
impl_bit_prop_for_int!(17, u128);
impl_bit_prop_for_int!(18, u128);
impl_bit_prop_for_int!(19, u128);
impl_bit_prop_for_int!(20, u128);
impl_bit_prop_for_int!(21, u128);
impl_bit_prop_for_int!(22, u128);
impl_bit_prop_for_int!(23, u128);
impl_bit_prop_for_int!(24, u128);
impl_bit_prop_for_int!(25, u128);
impl_bit_prop_for_int!(26, u128);
impl_bit_prop_for_int!(27, u128);
impl_bit_prop_for_int!(28, u128);
impl_bit_prop_for_int!(29, u128);
impl_bit_prop_for_int!(30, u128);
impl_bit_prop_for_int!(31, u128);
impl_bit_prop_for_int!(32, u128);
impl_bit_prop_for_int!(33, u128);
impl_bit_prop_for_int!(34, u128);
impl_bit_prop_for_int!(35, u128);
impl_bit_prop_for_int!(36, u128);
impl_bit_prop_for_int!(37, u128);
impl_bit_prop_for_int!(38, u128);
impl_bit_prop_for_int!(39, u128);
impl_bit_prop_for_int!(40, u128);
impl_bit_prop_for_int!(41, u128);
impl_bit_prop_for_int!(42, u128);
impl_bit_prop_for_int!(43, u128);
impl_bit_prop_for_int!(44, u128);
impl_bit_prop_for_int!(45, u128);
impl_bit_prop_for_int!(46, u128);
impl_bit_prop_for_int!(47, u128);
impl_bit_prop_for_int!(48, u128);
impl_bit_prop_for_int!(49, u128);
impl_bit_prop_for_int!(50, u128);
impl_bit_prop_for_int!(51, u128);
impl_bit_prop_for_int!(52, u128);
impl_bit_prop_for_int!(53, u128);
impl_bit_prop_for_int!(54, u128);
impl_bit_prop_for_int!(55, u128);
impl_bit_prop_for_int!(56, u128);
impl_bit_prop_for_int!(57, u128);
impl_bit_prop_for_int!(58, u128);
impl_bit_prop_for_int!(59, u128);
impl_bit_prop_for_int!(60, u128);
impl_bit_prop_for_int!(61, u128);
impl_bit_prop_for_int!(62, u128);
impl_bit_prop_for_int!(63, u128);
impl_bit_prop_for_int!(64, u128);
impl_bit_prop_for_int!(65, u128);
impl_bit_prop_for_int!(66, u128);
impl_bit_prop_for_int!(67, u128);
impl_bit_prop_for_int!(68, u128);
impl_bit_prop_for_int!(69, u128);
impl_bit_prop_for_int!(70, u128);
impl_bit_prop_for_int!(71, u128);
impl_bit_prop_for_int!(72, u128);
impl_bit_prop_for_int!(73, u128);
impl_bit_prop_for_int!(74, u128);
impl_bit_prop_for_int!(75, u128);
impl_bit_prop_for_int!(76, u128);
impl_bit_prop_for_int!(77, u128);
impl_bit_prop_for_int!(78, u128);
impl_bit_prop_for_int!(79, u128);
impl_bit_prop_for_int!(80, u128);
impl_bit_prop_for_int!(81, u128);
impl_bit_prop_for_int!(82, u128);
impl_bit_prop_for_int!(83, u128);
impl_bit_prop_for_int!(84, u128);
impl_bit_prop_for_int!(85, u128);
impl_bit_prop_for_int!(86, u128);
impl_bit_prop_for_int!(87, u128);
impl_bit_prop_for_int!(88, u128);
impl_bit_prop_for_int!(89, u128);
impl_bit_prop_for_int!(90, u128);
impl_bit_prop_for_int!(91, u128);
impl_bit_prop_for_int!(92, u128);
impl_bit_prop_for_int!(93, u128);
impl_bit_prop_for_int!(94, u128);
impl_bit_prop_for_int!(95, u128);
impl_bit_prop_for_int!(96, u128);
impl_bit_prop_for_int!(97, u128);
impl_bit_prop_for_int!(98, u128);
impl_bit_prop_for_int!(99, u128);
impl_bit_prop_for_int!(100, u128);
impl_bit_prop_for_int!(101, u128);
impl_bit_prop_for_int!(102, u128);
impl_bit_prop_for_int!(103, u128);
impl_bit_prop_for_int!(104, u128);
impl_bit_prop_for_int!(105, u128);
impl_bit_prop_for_int!(106, u128);
impl_bit_prop_for_int!(107, u128);
impl_bit_prop_for_int!(108, u128);
impl_bit_prop_for_int!(109, u128);
impl_bit_prop_for_int!(110, u128);
impl_bit_prop_for_int!(111, u128);
impl_bit_prop_for_int!(112, u128);
impl_bit_prop_for_int!(113, u128);
impl_bit_prop_for_int!(114, u128);
impl_bit_prop_for_int!(115, u128);
impl_bit_prop_for_int!(116, u128);
impl_bit_prop_for_int!(117, u128);
impl_bit_prop_for_int!(118, u128);
impl_bit_prop_for_int!(119, u128);
impl_bit_prop_for_int!(120, u128);
impl_bit_prop_for_int!(121, u128);
impl_bit_prop_for_int!(122, u128);
impl_bit_prop_for_int!(123, u128);
impl_bit_prop_for_int!(124, u128);
impl_bit_prop_for_int!(125, u128);
impl_bit_prop_for_int!(126, u128);
impl_bit_prop_for_int!(127, u128);
impl_bit_prop_for_int!(128, u128);

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

pub trait FieldAccessorScriptingGetPath {
    fn _scripting_get_path(&self) -> Cow<'static, str>;
}

pub trait FieldAccessor: FieldAccessorScriptingGetPath {
    #[allow(private_bounds)]
    type BoolArray: MustBeABoolArrayConstGenericsWorkaround;
    type Output: BitPropertyWithStringConv<Self::BoolArray, Self>
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
            bitstream.set(x, y, bits.as_ref()[biti]);
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

pub trait FieldAccessorDyn {
    fn _scripting_get_path(&self) -> Cow<'static, str>;

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

    fn _scripting_get_path(&self) -> Cow<'static, str> {
        A::_scripting_get_path(self)
    }
}

pub trait AccessorScriptingMetadata {
    fn _scripting_fields(&self) -> &'static [&'static str];
    fn _scripting_sublevels(&self) -> &'static [&'static str];

    fn _scripting_construct_field(&self, idx: usize, params: &[&str]) -> Box<dyn FieldAccessorDyn>;
    fn _scripting_descend_sublevel(
        &self,
        idx: usize,
        params: &[&str],
    ) -> Box<dyn AccessorScriptingMetadata>;
}

// pub struct TestBitstreamAllFieldsIter {
//     idx: usize,
// }
// impl Iterator for TestBitstreamAllFieldsIter {
//     type Item = Box<dyn FieldAccessorDyn>;

//     fn next(&mut self) -> Option<Self::Item> {
//         const STUFF: &'static [&'static dyn FnOnce() -> ()] = &[];
//         let idx = self.idx;
//         if idx >= STUFF.len() {
//             None
//         } else {
//             self.idx += 1;
//             Some(STUFF[idx])
//         }
//     }
// }

impl TestBitstream {
    fn _scripting_iter_all_stuff() -> impl Iterator<Item = Box<dyn FieldAccessorDyn>> {
        (0..4)
            .cartesian_product(0..4)
            .map(|(y, x)| {
                [
                    Box::new(Box::new(Tile::tile(x, y).property_one()))
                        as Box<dyn FieldAccessorDyn>,
                    Box::new(Box::new(Tile::tile(x, y).property_three()))
                        as Box<dyn FieldAccessorDyn>,
                    Box::new(Box::new(Tile::tile(x, y).property_four()))
                        as Box<dyn FieldAccessorDyn>,
                ]
                .into_iter()
                .chain((0..4).map(move |n| {
                    Box::new(Box::new(Tile::tile(x, y).property_two(n)))
                        as Box<dyn FieldAccessorDyn>
                }))
            })
            .flatten()
    }
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

    fn _scripting_descend_sublevel(
        &self,
        _idx: usize,
        _params: &[&str],
    ) -> Box<dyn AccessorScriptingMetadata> {
        unreachable!()
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
impl FieldAccessorScriptingGetPath for TilePropertyOneAccessor {
    fn _scripting_get_path(&self) -> Cow<'static, str> {
        format!("tile[{}, {}].property_one", self.tile.x, self.tile.y).into()
    }
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
impl FieldAccessorScriptingGetPath for TilePropertyTwoAccessor {
    fn _scripting_get_path(&self) -> Cow<'static, str> {
        format!(
            "tile[{}, {}].property_two[{}]",
            self.tile.x, self.tile.y, self.n
        )
        .into()
    }
}

pub struct CustomBool(bool);
impl BitProperty<[bool; 1]> for CustomBool {
    fn from_bits(bits: &[bool; 1]) -> Self {
        CustomBool(bits[0])
    }

    fn to_bits(&self) -> [bool; 1] {
        [self.0]
    }
}
impl BitPropertyWithStringConv<[bool; 1], TilePropertyThreeAccessor> for CustomBool {
    fn to_string(&self, accessor: &TilePropertyThreeAccessor) -> Cow<'static, str> {
        if !self.0 {
            "nonono".into()
        } else {
            format!("({}, {})", accessor.tile.x, accessor.tile.y).into()
        }
    }

    fn from_string(s: &str, _accessor: &TilePropertyThreeAccessor) -> Self {
        if s == "nonono" {
            Self(false)
        } else {
            Self(true)
        }
    }
}
impl BitPropertyWithStringConv<[bool; 1], TilePropertyFourAccessor> for CustomBool {
    fn to_string(&self, accessor: &TilePropertyFourAccessor) -> Cow<'static, str> {
        if !self.0 {
            "lalala".into()
        } else {
            format!("[{}, {}]", accessor.tile.x, accessor.tile.y).into()
        }
    }

    fn from_string(s: &str, _accessor: &TilePropertyFourAccessor) -> Self {
        if s == "lalala" {
            Self(false)
        } else {
            Self(true)
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
impl FieldAccessorScriptingGetPath for TilePropertyThreeAccessor {
    fn _scripting_get_path(&self) -> Cow<'static, str> {
        format!("tile[{}, {}].property_three", self.tile.x, self.tile.y).into()
    }
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
impl FieldAccessorScriptingGetPath for TilePropertyFourAccessor {
    fn _scripting_get_path(&self) -> Cow<'static, str> {
        format!("tile[{}, {}].property_four", self.tile.x, self.tile.y).into()
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

    #[test]
    fn test_scripting() {
        let bitstream = TestBitstream { bits: [false; 256] };
        let x = TestBitstream::_scripting_iter_all_stuff().collect::<Vec<_>>();
        for xi in x {
            let path = xi._scripting_get_path();
            let result = xi._scripting_get(&bitstream);
            println!("{} = {}", path, result);
        }
    }
}
