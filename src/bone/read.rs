use super::*;

fn read_at<'a, F, O>(i0: &'a [u8], f: F) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], O>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
{
    move |i: &'a [u8]| {
        let (i, ptr) = le_usize(i)?;
        trace!("read_at ptr: {}", ptr);
        f(&i0[ptr..]).map(|(_, val)| (i, val))
    }
}

fn read_count_at<'a, F, O>(i0: &'a [u8], f: F) -> impl FnOnce(&'a [u8]) -> IResult<&'a [u8], Vec<O>>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
{
    move |i: &'a [u8]| {
        let (i, ptr) = le_usize(i)?;
        let (i, cnt) = le_usize(i)?;
        trace!("read_count_at ptr: {}", ptr);
        count(f, cnt)(&i0[ptr..]).map(|(_, val)| (i, val))
    }
}

fn le_usize(i: &[u8]) -> IResult<&[u8], usize> {
    map(le_u32, |x| x as usize)(i)
}

fn string<'a>(i: &'a [u8]) -> IResult<&'a [u8], Cow<'a, str>> {
    map(is_not("\x00"), |x| String::from_utf8_lossy(x))(i)
}

fn offset_string<'a>(i0: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Cow<'a, str>> {
    read_at(i0, string)
}

impl<'a> BoneDatabase<'a> {
    pub fn read(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        let (i0, signature) = le_u32(i)?;
        let (i0, skel_cnt) = le_usize(i0)?;
        let (i0, skel_ptr) = le_usize(i0)?;
        let (i0, skel_name_ptr) = le_usize(i0)?;
        debug!(
            "bonedb
name, count*ptr
signature: {}
skeleton: {} * {}
skeleton_names: {} * {}",
            signature, skel_cnt, skel_ptr, skel_cnt, skel_name_ptr
        );

        let (_, skeletons) = count(read_at(i, Skeleton::read(i)), skel_cnt)(&i[skel_ptr..])?;
        let (_, skeleton_names) = count(offset_string(i), skel_cnt)(&i[skel_name_ptr..])?;
        debug!("bonedb {:#?}", skeleton_names);

        let skeletons = skeletons
            .into_iter()
            .zip(skeleton_names.into_iter())
            .map(|(s, name)| Skeleton { name, ..s })
            .collect();
        Ok((
            i0,
            BoneDatabase {
                signature,
                skeletons,
            },
        ))
    }
}

impl<'a> Skeleton<'a> {
    pub fn read(i0: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Skeleton<'a>> {
        move |i: &'a [u8]| {
            let (i, bone_ptr) = le_usize(i)?;

            let count_ptr = tuple((le_usize, le_usize));

            let (i, (pos_cnt, pos_ptr)) = count_ptr(i)?;
            let (i, _unk_ptr) = le_usize(i)?;
            let (i, (obj_cnt, obj_ptr)) = count_ptr(i)?;
            let (i, (mot_cnt, mot_ptr)) = count_ptr(i)?;
            let (i, parent_id_ptr) = le_usize(i)?;
            debug!(
                "skel
    name, count*ptr
    bone: {:#X}
    pos: {}*{:#X?}
    obj: {}*{:#X?}
    mot: {}*{:#X?}
    parent : {}*{:#X?}",
                bone_ptr,
                pos_cnt,
                pos_ptr,
                obj_cnt,
                obj_ptr,
                mot_cnt,
                mot_ptr,
                mot_cnt,
                parent_id_ptr
            );

            let (_, bones) = nom::multi::many0(Bone::read(i0))(&i0[bone_ptr..])?;
            trace!("[SUCESS] Bone read");
            let vec3 = map(tuple((le_f32, le_f32, le_f32)), |(x, y, z)| [x, y, z]);
            let (_, pos) = count(vec3, pos_cnt)(&i0[pos_ptr..])?;
            //println!("vec3: {:.02?}", pos);
            trace!("[SUCESS] Position read");
            let (_, object_bone_names) = count(offset_string(i0), obj_cnt)(&i0[obj_ptr..])?;
            trace!("[SUCESS] Object bone names read");
            trace!("obj names: {:?}", object_bone_names);
            let (_, motion_bone_names) = count(offset_string(i0), mot_cnt)(&i0[mot_ptr..])?;
            trace!("[SUCESS] Motion bone names read");
            trace!("mot names: {:?}", object_bone_names);
            let (_, parent_ids) = count(le_i16, mot_cnt)(&i0[parent_id_ptr..])?;
            trace!("[SUCESS] Parent ids read");

            let skel = Skeleton {
                bones,
                pos,
                object_bone_names,
                motion_bone_names,
                parent_ids,
                ..Default::default()
            };
            Ok((i, skel))
        }
    }
}

impl<'a> Bone<'a> {
    pub fn read(i0: &'a [u8]) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Bone<'a>> {
        use nom::combinator::map_opt;
        move |i: &'a [u8]| {
            trace!("Mode read: {}", i[0]);
            let (_, mode) = map_opt(le_u8, |x| BoneType::from_int(x))(i)?;
            let parent = i[1] != 0;
            let parent = if parent { Some(i[2]) } else { None };
            let pole_target = if i[3] != 0 { Some(i[3] )} else { None };
            let mirror = if i[4] != 255 { Some(i[4]) } else { None };
            let has_unk = i[5] != 0;
            let unk2 = if has_unk { Some(i[6]) } else { None };
            let unk2 = i[5];
            // let unk = [i[2], i[3], i[4], i[6]];
            //seek 2 bytes then read ptr
            let (i, name) = offset_string(i0)(&i[8..])?;
            trace!("bone name={}", name);
            Ok((
                i,
                Bone {
                    mode,
                    parent,
                    pole_target,
                    mirror,
                    unk2,
                    name,
                },
            ))
        }
    }
}
