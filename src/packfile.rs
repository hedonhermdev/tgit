use anyhow::Result;
use std::io::{BufRead, Cursor, Read};
use u8;

enum ObjectType {}

fn read_object_size(c: &mut Cursor<&[u8]>) -> Result<usize> {
    let mut size_vec: Vec<u8> = vec![];

    let mut keep_reading = true;

    loop {
        if !keep_reading {
            println!("LOOP BREAK");
            break;
        }

        let mut one_byte: [u8; 1] = [0];

        c.read_exact(&mut one_byte)?;

        println!("Before {:b}", one_byte[0]);

        if one_byte[0] >> 7u8 == 0 {
            keep_reading = false;
        } else {
            one_byte[0] = one_byte[0] - 0b10000000;
        }
        println!("After {:8b}", one_byte[0]);
        size_vec.push(one_byte[0]);
    }

    size_vec.reverse();

    println!("{:x?}", size_vec);
    Ok(0usize)
}

pub struct Packfile();

impl Packfile {
    pub fn parse_data(data: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(data.clone());

        let mut signature: [u8; 4] = [0; 4];
        println!("{:x?}", signature);

        cursor.read_exact(&mut signature)?;

        let mut version_num: [u8; 4] = [0; 4];

        cursor.read_exact(&mut version_num)?;

        loop {
            let size = read_object_size(&mut cursor)?;

            break;
        }

        Ok(Self())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_packfile() -> Result<()> {
        let data = fs::read(
            "./tempgit/.git/objects/pack/pack-4e3c870cda81214366531c32ed63a52dbebc56fd.pack",
        )?;
        println!("{:x?}", data);
        let packfile = Packfile::parse_data(&data);

        Ok(())
    }
}
