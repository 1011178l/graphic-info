use std::fs::File;
use std::io::Read;
use crate::graphic_info::GraphicInfo;

#[derive(Debug)]
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

    pub fn show_info (&mut self) {
        println!("Number of Graphic Info: {}", self.count());
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

#[cfg(test)]
mod tests {
    use super::{GraphicInfoFile, GraphicInfo};

    #[test]
    fn test_new () {
        assert!(GraphicInfoFile::new("resources/GraphicInfo.test.bin").is_ok());
    }

    #[test]
    fn test_new_failed() {
        assert!(GraphicInfoFile::new("resources/GraphicInfo-broken.test.bin").is_err())
    }

    #[test]
    fn test_iter() {
        let file = GraphicInfoFile::new("resources/GraphicInfo.test.bin").unwrap();
        let blocks: Vec<GraphicInfo> = file.collect();

        assert_eq!(3, blocks.len());
    }
}