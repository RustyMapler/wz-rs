use crate::{WzFile, WzNode, WzVersion};
use std::io::Error;

pub struct WzFileSet {
    pub files: Vec<WzFile>,
    version: Option<i16>,
    is_ready: bool,
}

impl WzFileSet {
    pub fn from_paths(paths: Vec<String>, version: WzVersion) -> WzFileSet {
        let mut files: Vec<WzFile> = vec![];

        for path in paths {
            let wz_file = WzFile::new(&path, version);
            files.push(wz_file);
        }

        WzFileSet {
            files,
            version: None,
            is_ready: false,
        }
    }

    pub fn open(&mut self) -> Result<(), Error> {
        for file in self.files.iter_mut() {
            file.open()?;
        }
        self.is_ready = true;
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    pub fn resolve(&mut self, path: &str) -> Option<&mut dyn WzNode> {
        // Set all versions, if a file version was found in this set
        // if self.version.is_none() {
        //     if let Some(version) = self.find_version() {
        //         self.apply_version(version);
        //     }
        // }

        // Resolve path in all files
        for file in self.files.iter_mut() {
            let node = file.resolve(path);
            if node.is_some() {
                return node;
            }
        }

        None
    }

    // Find the file version from any file in the set
    fn find_version(&self) -> Option<i16> {
        for file in self.files.iter() {
            if file.version > 0 {
                return Some(file.version);
            }
        }

        None
    }

    // Apply the file version to all files in the set
    fn apply_version(&mut self, version: i16) {
        self.version = Some(version);
        for file in self.files.iter_mut() {
            file.set_version(version);
        }
    }
}
