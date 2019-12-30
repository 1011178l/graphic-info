use std::fs::File;
use std::io::Read;
use crate::graphic_info::GraphicInfo;

pub struct GraphicInfoFile {
    file: File,
}

impl GraphicInfoFile {
    pub fn new (path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(&path)?;

        if file.metadata()?.len() % 40 != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid input file size."));
        }

        Ok(Self{file})
    }
}

impl Iterator for GraphicInfoFile {
    type Item = GraphicInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 40];

        match self.file.read_exact(&mut buf) {
            Ok(_) => return Some(GraphicInfo::new(&buf).unwrap()),
            Err(_) => return None,
        }
    }
}