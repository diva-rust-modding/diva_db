use nom_ext::*;
#[cfg(feature = "serde")]
use serde::*;
#[cfg(feature="pyo3")]
use pyo3::{prelude::*, wrap_pyfunction, PyObjectProtocol};

use std::{borrow::Cow, collections::BTreeMap};

use super::*;

#[cfg(test)]
mod tests;
#[cfg(feature="pyo3")]
pub mod py_ffi;

// #[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MotionSetInfo<'a> {
    name: Cow<'a, str>,
    mots: BTreeMap<u32, Cow<'a, str>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MotionSetDatabase<'a> {
    pub signature: u32,
    pub sets: BTreeMap<u32, MotionSetInfo<'a>>,
    pub bones: Vec<Cow<'a, str>>,
}

use nom::multi::count;
use nom::number::complete::*;
use nom::number::Endianness;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::IResult;

impl<'a> MotionSetInfo<'a> {
    pub fn read(i0: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], MotionSetInfo<'a>> {
        move |i: &[u8]| {
            let endian = Endianness::Little;
            let (i, name) = offset_string(i0, endian)(i)?;
            let (i, mot_name_table_offset) = u32_usize(endian)(i)?;
            let (i, mot_count) = u32_usize(endian)(i)?;
            let (i, mot_ids_offset) = u32_usize(endian)(i)?;

            let (_, mot_ids) = at_offset(mot_ids_offset, count(le_u32, mot_count))(i0)?;
            let (_, mot_names) = at_offset(mot_name_table_offset, count(offset_string(i0, endian), mot_count))(i0)?;

            let mots = mot_ids.into_iter().zip(mot_names.into_iter()).collect();
            Ok((i, Self { name, mots}))
        }
    }
}

impl<'a> MotionSetDatabase<'a> {
    pub fn read(i0: &'a [u8]) -> IResult<&'a [u8], MotionSetDatabase<'a>> {
        let endian = Endianness::Little;
        let (i, signature) = le_u32(i0)?;
        let (i, set_infos_off) = u32_usize(endian)(i)?;
        let (i, set_ids_off) = u32_usize(endian)(i)?;
        let (i, set_infos_cnt) = u32_usize(endian)(i)?;
        let (i, bone_name_table_offset) = u32_usize(endian)(i)?;
        let (i, bone_name_count) = u32_usize(endian)(i)?;

        let (_, sets) = at_offset(set_infos_off, count(MotionSetInfo::read(i0), set_infos_cnt))(i0)?;
        let (_, set_ids) = at_offset(set_ids_off, count(le_u32, set_infos_cnt))(i0)?;

        let sets = set_ids.into_iter().zip(sets.into_iter()).collect();

        let (_, bones) = at_offset(bone_name_table_offset, count(offset_string(i0, endian), bone_name_count))(i0)?;

        Ok((i, Self { signature, sets, bones}))
    }
}
