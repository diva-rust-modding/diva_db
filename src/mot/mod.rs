use nom_ext::*;

use super::*;

use std::borrow::Cow;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct MotionInfo<'a> {
    name: Cow<'a, str>,
    id: u32,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct MotionSetInfo<'a> {
    name: Cow<'a, str>,
    id: u32,
    mots: Vec<MotionInfo<'a>>,
}

#[derive(Debug)]
pub struct MotionSetDatabase<'a> {
    pub sets: Vec<MotionSetInfo<'a>>,
    pub bones: Vec<Cow<'a, str>>,
}

use nom::multi::count;
use nom::number::complete::*;
use nom::number::Endianness;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::IResult;

impl<'a> MotionSetInfo<'a> {
    pub fn read(i0: &'a [u8], endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], MotionSetInfo<'a>> {
        move |i: &[u8]| {
            let (i, name) = offset_string(i0, endian)(i)?;
            let (i, mot_name_table_offset) = u32_usize(endian)(i)?;
            let (i, mot_count) = u32_usize(endian)(i)?;
            let (i, mot_ids) = offset_then(i0, count(u32(endian), mot_count), endian)(i)?;

            let (i, mot_names) = at_offset(mot_name_table_offset, count(offset_string(i0, endian), mot_count))(i)?;

            let mots = mot_ids.into_iter().zip(mot_names.into_iter()).map(|(id, name)| MotionInfo { id, name: name}).collect();
            Ok((i, Self { id: 0, name: name, mots}))
        }
    }
}

impl<'a> MotionSetDatabase<'a> {
    pub fn read(endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], MotionSetDatabase<'a>> {
        move |i0| {
            let (i, ver) = u32_usize(endian)(i0)?;
            let (i, set_infos_off) = u32_usize(endian)(i)?;
            let (i, set_ids_off) = u32_usize(endian)(i)?;
            let (i, set_infos_cnt) = u32_usize(endian)(i)?;
            let (i, bone_name_table_offset) = u32_usize(endian)(i)?;
            let (i, bone_name_count) = u32_usize(endian)(i)?;

            let (_, sets) = at_offset(set_infos_off, count(MotionSetInfo::read(i0, endian), set_infos_cnt))(i0)?;
            let mut sets = sets;
            let (_, set_ids) = at_offset(set_ids_off, count(u32(endian), set_infos_cnt))(i0)?;
            for (set, id) in sets.iter_mut().zip(set_ids.into_iter()) {
                set.id = id;
            }

            let (_, bones) = at_offset(bone_name_table_offset, count(offset_string(i0, endian), bone_name_count))(i0)?;

            Ok((i, Self { sets, bones}))
        }
    }
}