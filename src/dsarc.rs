use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct HeaderEntry {
    pub filename: String,
    pub size: usize,
    pub offset: usize,
}

#[derive(Debug)]
pub struct Header {
    pub entries: Vec<HeaderEntry>
}

#[derive(Debug)]
pub struct Archive {
    pub header: Header,
    pub data: Vec<Vec<u8>>
}

pub mod parser {
    use super::*;
    use nom::number::streaming::le_u32;

    named!(c_string<&[u8], &str>,
           map_res!(terminated!(take_while!(|b: u8| b!=0), tag!([0])), std::str::from_utf8)
    );

    named!(header_entry<&[u8], HeaderEntry>,
           do_parse!(
               filename: map_res!(take!(112), c_string) >>
                   unk1: le_u32 >>
                   size: le_u32 >>
                   offset: le_u32 >>
                   unk2: le_u32 >>
                   (HeaderEntry {
                       filename: String::from(filename.1),
                       size: size as usize,
                       offset: offset as usize,
                   })
           )
    );

    named!(pub header<&[u8], Header>,
           do_parse!(
               tag!("DSARC FL") >>
                   entry_count: le_u32 >>
                   pad: take!(0x4) >>
                   entries: count!(header_entry, entry_count as usize) >>
                   (Header { entries })
           )
    );
}

pub fn load<T: AsRef<Path>>(filepath: T) -> Result<Archive> {
    match File::open(filepath.as_ref()) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).expect("Unable to read file");
            load_bytes(&buffer)
        }
        Err(e) => Err(anyhow!("Unable to load file: {}", e)),
    }
}

pub fn load_bytes(bytes: &[u8]) -> Result<Archive> {
    let (_, header) = parser::header(bytes).expect("Not a valid DSARC FL archive");

    let mut data = Vec::new();

    for entry in header.entries.iter() {
        println!("{} {} {}", entry.filename, entry.offset, entry.size);
        let entry_data = Vec::from(&bytes[entry.offset..entry.offset+entry.size]);
        data.push(entry_data);
    }

    print_trace!();

    Ok(Archive { header, data })
}
