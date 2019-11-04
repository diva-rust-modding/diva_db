use crate::*;
use int_enum::*;
#[cfg(feature="serde")]
use serde::*;
use nom::{
    bytes::complete::is_not, combinator::map,
    multi::*, number::complete::*, sequence::tuple, *,
};
use smart_default::*;

use std::borrow::Cow;

#[cfg(test)]
mod tests;
mod read;

type Vec3 = (f32, f32, f32);

#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct BoneDatabase<'a> {
    pub signature: u32,
    pub skeletons: Vec<Skeleton<'a>>,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Skeleton<'a> {
    pub name: Cow<'a, str>,
    pub bones: Vec<Bone<'a>>,
    pub pos: Vec<Vec3>,
    pub parent_ids: Vec<i16>,

    pub object_bone_names: Vec<Cow<'a, str>>,
    pub motion_bone_names: Vec<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
pub struct Bone<'a> {
    pub mode: BoneType,
    pub parent: bool,
    pub unk: [u8; 4],
    pub name: Cow<'a, str>,
}

#[int_enum(u8)]
#[derive(Debug, SmartDefault, PartialEq, PartialOrd)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
//Need to explictly set variant numbers for int_enum to work
pub enum BoneType {
    #[default]
    Rotation = 0,
    Type1 = 1,
    Position = 2,
    Type3 = 3,
    Type4 = 4,
    Type5 = 5,
    Type6 = 6,
}
