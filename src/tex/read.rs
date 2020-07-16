use nom::IResult;
use nom::number::Endianness;
use nom::sequence::tuple;
use nom_ext::*;

use super::*;

impl<'a> TextureDatabase<'a> {
    pub fn parse(i: &'a [u8]) -> IResult<&'a [u8], Self> {
        let entry = tuple((u32_usize(Endianness::Little), offset_string(i, Endianness::Little)));
        let (i, entries) = count_then_offset(i, u32_usize(Endianness::Little), entry)(i)?;
        let entries = entries.into_iter().collect();
        Ok((i, Self { entries }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod read {
        use super::*;

        const INPUT: &[u8] = include_bytes!("../../assets/aft_tex_db.bin");

        #[test]
        fn texture_db() {
            let (_, db) = TextureDatabase::parse(INPUT).unwrap();
            println!("{:#?}", db);
            panic!()
        }
    }
}
