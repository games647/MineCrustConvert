extern crate nbtrs;
extern crate flate2;
extern crate uuid;
extern crate md5;

use uuid::Uuid;
use uuid::UuidVersion;

use nbtrs::Tag;

use std::fs::File;
use std::fs;
use std::str::FromStr;
use std::fs::ReadDir;
use std::fmt::Write;

use flate2::read::GzDecoder;

use md5::Context;

fn get_playername(s: &str) -> Option<String> {
    let file: File = File::open(s).unwrap();
    let mut decoder = GzDecoder::new(file).unwrap();
    let (_, root_tag) : (String, Tag) = Tag::parse(&mut decoder).unwrap();

    match root_tag {
        Tag::TagCompound(ref map) => {
            let bukkit_tag = map.get(&"bukkit".to_string()).unwrap();
            match bukkit_tag {
                &Tag::TagCompound(ref map) => {
                    let name_tag: &Tag = map.get(&"lastKnownName".to_string()).unwrap();
                    match name_tag {
                        &Tag::TagString(ref username) => {
                            return Some(username.clone())
                        },
                        _ => panic!("Unknown value"),
                    }
                },
                _ => panic!("Bukkit NBT tag doesn't exist"),
            }
        },
        _ => println!("No data"),
    }

    return None;
}

fn main() {
    println!("{:?}", gen_offline_uuid("games647"));
    let files : ReadDir = fs::read_dir("./").unwrap();

    for file in files {
        let dir_entry: fs::DirEntry = file.unwrap();
        let file_type: fs::FileType = dir_entry.file_type().unwrap();

        if file_type.is_file() {
            let os_name = dir_entry.file_name();
            let file_name: &str = os_name.to_str().unwrap();

            if file_name.ends_with(".dat") {
                let playername = get_playername(file_name).unwrap();
                println!("Converting {}", playername);
                fs::rename(file_name, format!("{}.dat", gen_offline_uuid(&playername)));
            }
        }
    }
}

fn gen_offline_uuid(username: &str) -> String {
    let key = format!("{}{}", "OfflinePlayer:", username);
    let mut hash = md5::compute(key.as_bytes());
    //set the version to 3 -> Name based md5 hash
    hash[6] = hash[6] & 0x0f | 0x30;
    //IETF variant
    hash[8] = hash[8] & 0x3f | 0x80;

    return tohex(&hash)
}

fn tohex(input: &[u8]) -> String {
    #[inline]
    fn hex(d: u8) -> char {
        match d {
            0...9 => (d + 0x30) as char,
            10...15 => (d + 0x57) as char,
            _ => unreachable!("unexpected value: {}", d),
        }
    }

    let mut buf = String::with_capacity(32);
    for b in input.into_iter() {
        buf.push(hex(b >> 4));
        buf.push(hex(b & 0xf));
    }

    buf
}