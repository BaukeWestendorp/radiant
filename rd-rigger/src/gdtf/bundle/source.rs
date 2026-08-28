use std::{
    collections::HashMap,
    fs,
    io::{self, BufReader},
    path::PathBuf,
};

use crate::gdtf::bundle::{Bundle, DESCRIPTION_FILE_NAME};

pub(crate) trait SourceLoader {
    fn load_bundle(&self) -> Bundle;
}

pub(crate) struct FolderSource {
    path: PathBuf,
}

impl FolderSource {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl SourceLoader for FolderSource {
    fn load_bundle(&self) -> Bundle {
        let mut deserializer = quick_xml::de::Deserializer::from_reader(BufReader::new(
            fs::File::open(self.path.join(DESCRIPTION_FILE_NAME)).unwrap(),
        ));
        let description = serde_path_to_error::deserialize(&mut deserializer).unwrap();

        let mut resources = HashMap::new();

        let mut dirs = vec![self.path.clone()];
        while let Some(dir) = dirs.pop() {
            let Ok(read_dir) = fs::read_dir(&dir) else { continue };
            for entry in read_dir.flatten() {
                let p = entry.path();

                if p.is_dir() {
                    dirs.push(p);
                    continue;
                }

                if !p.is_file() {
                    continue;
                }

                let relative = match p.strip_prefix(&self.path) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let bytes = std::fs::read(self.path.join(relative)).unwrap();
                resources.insert(relative.to_path_buf(), bytes);
            }
        }

        Bundle { description, resources, path: Some(self.path.clone()) }
    }
}

pub(crate) struct ArchiveSource {
    path: PathBuf,
}

impl ArchiveSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl SourceLoader for ArchiveSource {
    fn load_bundle(&self) -> Bundle {
        let file = std::fs::File::open(&self.path).unwrap();
        let mut zip = zip::ZipArchive::new(file).unwrap();

        let mut resources = HashMap::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let name = file.name();
            if name == DESCRIPTION_FILE_NAME {
                continue;
            }
            let mut bytes = Vec::new();
            io::Read::read_to_end(&mut file, &mut bytes).unwrap();
            let name = String::from_utf8_lossy(file.name_raw()).to_string();
            resources.insert(PathBuf::from(name), bytes);
        }

        let desc_file = zip.by_name(DESCRIPTION_FILE_NAME).unwrap();
        let mut deserializer =
            quick_xml::de::Deserializer::from_reader(io::BufReader::new(desc_file));
        let description = serde_path_to_error::deserialize(&mut deserializer).unwrap();

        Bundle { description, resources, path: Some(self.path.clone()) }
    }
}

pub(crate) struct ArchiveBytesSource<'a> {
    bytes: &'a [u8],
}

impl<'a> ArchiveBytesSource<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}

impl SourceLoader for ArchiveBytesSource<'_> {
    fn load_bundle(&self) -> Bundle {
        let mut zip = zip::ZipArchive::new(io::Cursor::new(self.bytes)).unwrap();

        let mut resources = HashMap::new();
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let name = file.name();
            if name == DESCRIPTION_FILE_NAME {
                continue;
            }
            let mut bytes = Vec::new();
            io::Read::read_to_end(&mut file, &mut bytes).unwrap();
            let name = String::from_utf8_lossy(file.name_raw()).to_string();
            resources.insert(PathBuf::from(name), bytes);
        }

        let desc_file = zip.by_name(DESCRIPTION_FILE_NAME).unwrap();
        let mut deserializer =
            quick_xml::de::Deserializer::from_reader(io::BufReader::new(desc_file));
        let description = serde_path_to_error::deserialize(&mut deserializer).unwrap();

        Bundle { description, resources, path: None }
    }
}
