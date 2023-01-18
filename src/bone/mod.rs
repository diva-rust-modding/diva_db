use crate::*;
use nom::{
    bytes::complete::is_not, combinator::map, multi::*, number::complete::*, sequence::tuple, *,
};
#[cfg(feature = "serde")]
use serde::*;

use std::borrow::Cow;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
pub mod py_ffi;
mod read;
#[cfg(test)]
mod tests;

type Vec3 = [f32; 3];

#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoneDatabase<'a> {
    pub signature: u32,
    pub skeletons: Vec<Skeleton<'a>>,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Skeleton<'a> {
    pub name: Cow<'a, str>,
    pub bones: Vec<Bone<'a>>,
    pub pos: Vec<Vec3>,
    pub parent_ids: Vec<i16>,

    pub object_bone_names: Vec<Cow<'a, str>>,
    pub motion_bone_names: Vec<Cow<'a, str>>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bone<'a> {
    pub mode: BoneType,
    pub parent: Option<u8>,
    pub pole_target: Option<u8>, //only set in type 5 bones
    pub mirror: Option<u8>,
    pub unk2: u8,
    pub name: Cow<'a, str>,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum BoneType {
    #[default]
    Rotation = 0,
    Type1 = 1,
    Position = 2,
    PositionRotation = 3,
    HeadIk = 4,
    ArmIk = 5,
    LegIk = 6,
}
