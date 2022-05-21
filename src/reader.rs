use crate::list::{Error, List, Mod};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{fs::DirEntry, path::Path};

pub fn read(path: &Path) -> Result<List, Error> {
    let dirread = fs::read_dir(&path)?;
    let mut list = List::new();
    for dir in dirread {
        let dir = dir?;
        let m = mod_dir(dir)?;
        list.add(m);
    }
    Ok(list)
}

fn mod_dir(entry: DirEntry) -> Result<Mod, Error> {
    let reader = fs::read_dir(entry.path())?;
    for file in reader {
        let file = file?;
        if file.file_name() == "ServerData.json" {
            let f = File::open(file.path()).unwrap();
            let mut reader = BufReader::new(f);

            let mut content = vec![];
            let _ = reader.read_to_end(&mut content);

            let s = String::from_utf8(content).unwrap();
            let t = s.replace(|c: char| !c.is_ascii(), "");

            return Mod::from_str(&t);
        }
    }
    return Err(Error::File);
}
