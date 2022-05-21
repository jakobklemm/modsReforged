use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Error {
    Parser,
    File,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Parser => write!(f, "unable to parse"),
            _ => write!(f, "unknown error")
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(_err: serde_json::Error) -> Self {
        Self::Parser
    }
}

impl From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Self {
        Self::File
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub modId: String,
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerData {
    id: String,
    name: String,
    revision: Revision, 
}

#[derive(Serialize, Deserialize, Debug)]
struct Revision {
    version: String,
    dependencies: Vec<String>,
    scenarios: Vec<Scenario>,
    downloaded: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct Scenario {
    name: String,
    gameId: String,
}

impl ServerData {
    fn to_mod(&self) -> Mod {
        Mod {
            modId: self.id.clone(),
            name: self.name.clone(),
            version: self.revision.version.clone(),
        }
    }
}

impl Mod {
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        Self {
            modId: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn to_string(&self) -> Result<String, Error> {
        if let Ok(s) = serde_json::to_string(&self) {
            return Ok(s);
        } else {
            return Err(Error::Parser);
        }
    }
    
    pub fn from_str(data: &str) -> Result<Self, Error> {
        let r: Result<ServerData, serde_json::Error> = serde_json::from_str(data);
        if let Ok(s) = r {
            return Ok(s.to_mod());
        } else {
            return Err(Error::Parser);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportList(pub Vec<Mod>);

impl List {
    pub fn new() -> Self {
        Self { mods: Vec::new() }
    }

    pub fn add(&mut self, m: Mod) {
        self.mods.push(m);
    }

    pub fn to_string(&self) -> Result<String, Error> {
        if let Ok(s) = serde_json::to_string(&self) {
            return Ok(s);
        } else {
            return Err(Error::Parser);
        }
    }

    pub fn from_str(data: &str) -> Result<Self, Error> {
        if let Ok(s) = serde_json::from_str(data) {
            return Ok(s);
        } else {
            return Err(Error::Parser);
        }
    }

    pub fn from_vec(v: Vec<Mod>) -> Self {
        Self {
            mods: v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{BufReader, Read};
    
    #[test]
    fn test_parse_server() {
        let path = "/home/jeykey/Downloads/mods/addons/5614E482BF83E310/ServerData.json";
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);

        let mut content = vec![];
        let _ = reader.read_to_end(&mut content);


        let s = String::from_utf8(content).unwrap();
        let t = s.replace(|c: char| !c.is_ascii(), "");

        let p: ServerData = serde_json::from_str(&t).unwrap();
        assert_eq!(p.id, String::from("5614E482BF83E310",));
    }
}
