use crate::{
    get_iv_for_version, resolve_uol_path, wz_crypto::generate_wz_key, WzDirectory, WzNode,
    WzObject, WzReader, WzVersion,
};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Cursor, Error, ErrorKind, Read},
    path::Path,
};

const INVALID_VERSION: i16 = -1;
const MAX_BRUTE_FORCE_VERSION: i16 = 230;

pub struct WzFile {
    pub name: String,
    pub reader: Option<WzReader>,
    pub version: i16,
    pub version_hash: u32,
    pub root: Option<WzDirectory>,
    pub file_path: String,
    pub wz_version: WzVersion,
}

unsafe impl Send for WzFile {}
unsafe impl Sync for WzFile {}

impl WzFile {
    pub fn new(path: &str, version: WzVersion) -> WzFile {
        let file_path = Path::new(path);

        WzFile {
            name: file_path.file_name().unwrap().to_str().unwrap().into(),
            file_path: path.to_string(),
            reader: None,
            version: INVALID_VERSION,
            version_hash: 0,
            root: None,
            wz_version: version,
        }
    }

    /// Creates a WzFile from filepath
    pub fn open(&mut self) -> Result<(), Error> {
        log::trace!("name: {}", self.name);
        let file_path = Path::new(&self.file_path);

        let mut file = File::open(file_path)?;
        let metadata = fs::metadata(file_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)?;

        let cursor_file = Cursor::new(buffer);

        let mut reader = WzReader {
            file: cursor_file.into(),
            wz_key: generate_wz_key(get_iv_for_version(self.wz_version)),
            file_start: 0,
            hash: 0,
        };
        WzFile::parse_wz_header(&mut reader)?;

        self.reader = Some(reader);
        self.parse_wz_main_directory().unwrap();

        Ok(())
    }

    /// Find an object using its pathname
    pub fn resolve(&mut self, path: &str) -> Option<&mut dyn WzNode> {
        match self.root.as_mut().unwrap().resolve(path) {
            Some(node) => {
                if node.is_uol() {
                    let resolved_uol_path =
                        resolve_uol_path(path.to_string(), node.get_uol().unwrap());
                    return self.root.as_mut().unwrap().resolve(&resolved_uol_path);
                }

                self.root.as_mut().unwrap().resolve(path)
            }
            None => None,
        }
    }

    /// Parse the header for a .wz file. Set the file_start for the reader.
    fn parse_wz_header(reader: &mut WzReader) -> Result<(), Error> {
        let ident = reader.read_string(4)?;
        log::trace!("ident: {}", ident);

        if ident != "PKG1" {
            return Err(Error::new(ErrorKind::Other, "Invalid .wz file"));
        }

        let file_size = reader.read_u64()?;
        log::trace!("file size: {}", file_size);

        let file_start = reader.read_u32()?;
        log::trace!("file start: {}", file_start);
        reader.file_start = file_start;

        let copyright = reader.read_string_to_end()?;
        log::trace!("copyright: {}", copyright);

        Ok(())
    }

    /// Parse the main directory for a .wz file. Nodes can only be resolved when parsed first.
    fn parse_wz_main_directory(&mut self) -> Result<(), Error> {
        // Determine file version
        if !self.is_version_set() {
            let version_from_header = self.reader.as_mut().unwrap().read_u16()?;
            log::trace!("version from header: {}", version_from_header);

            // This is a known version, go ahead and test
            if self.detect_known_version(version_from_header) {
                const MAPLE_KNOWN_VERSION: i16 = 777;
                if let Some((version, hash)) = self.attempt_known_version(MAPLE_KNOWN_VERSION) {
                    log::trace!("success! patch version is v230 or greater!");
                    self.version = version;
                    self.version_hash = hash;
                } else {
                    log::trace!("failed to read patch version!");
                }
            } else {
                // If we're using this in a custom client, we'll never have a patch version
                // Brute force the patch version instead
                if let Some((version, hash)) = self.bruteforce_version(version_from_header as i16) {
                    log::trace!("success! patch version is {}", version);
                    self.version = version;
                    self.version_hash = hash;
                } else {
                    log::trace!("failed to read patch version!");
                }
            }
        }

        if !self.is_version_set() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Unable to determine version",
            ));
        }

        let offset = WzFile::get_offset_for_version(self.reader.as_ref().unwrap(), self.version);

        self.root = Some(WzDirectory {
            file: self,
            reader: self.reader.as_mut().unwrap(),
            offset,
            name: self.name.clone(),
            sub_directories: HashMap::new(),
            objects: HashMap::new(),
        });
        self.root.as_mut().unwrap().parse_directory(true).unwrap();

        Ok(())
    }

    // Set a known version. This will skip any version tests that haven't run yet.
    pub fn set_version(&mut self, version: i16) {
        self.version = version;
        self.version_hash = WzFile::calculate_version_hash(version);
        self.reader.as_mut().unwrap().hash = self.version_hash;
        log::trace!("setting version to {}", version);
    }

    // File offset depends on the version
    fn get_offset_for_version(reader: &WzReader, version: i16) -> u32 {
        if version > MAX_BRUTE_FORCE_VERSION {
            reader.file_start
        } else {
            reader.file_start + 2
        }
    }

    // For versions v230 or higher
    fn detect_known_version(&mut self, version_from_header: u16) -> bool {
        if version_from_header > 0xff {
            return true;
        }
        // // Some weird use case that we don't need (yet)
        // else if version_from_header == 0x80 {
        //     self.reader.seek(self.reader.file_start as u64).unwrap();
        //     let prop_count = self.reader.read_wz_int().unwrap();
        //     if prop_count > 0 && (prop_count & 0xFF) == 0 && prop_count <= 0xFFFF {
        //         return true;
        //     }
        // }
        false
    }

    // Get the version by testing a known version
    fn attempt_known_version(&mut self, version: i16) -> Option<(i16, u32)> {
        let version_hash = WzFile::calculate_version_hash(version);
        match self.verify_version_hash(version, version_hash) {
            Ok(_) => Some((version, version_hash)),
            Err(_) => None,
        }
    }

    // Get the version by testing all versions between 0 and MAX_BRUTE_FORCE_VERSION
    fn bruteforce_version(&mut self, version_from_header: i16) -> Option<(i16, u32)> {
        for version in 0..MAX_BRUTE_FORCE_VERSION {
            let version_hash = WzFile::calculate_version_hash(version);
            if WzFile::match_version_hash(version_from_header, version_hash) {
                match self.verify_version_hash(version, version_hash) {
                    Ok(_) => return Some((version, version_hash)),
                    Err(_) => continue,
                }
            }
        }

        None
    }

    // Returns if this version is set to a valid value
    fn is_version_set(&mut self) -> bool {
        self.version != INVALID_VERSION
    }

    // Calculate the hash from version number
    fn calculate_version_hash(version: i16) -> u32 {
        let version_str = version.to_string();

        let mut version_hash: u32 = 0;
        for c in version_str.chars() {
            version_hash = (32 * version_hash) + (c as u32) + 1;
        }

        version_hash
    }

    // Using the version hash, attempt to match the version from the file header
    fn match_version_hash(version_from_header: i16, version_hash: u32) -> bool {
        let a = (version_hash >> 24) & 0xFF;
        let b = (version_hash >> 16) & 0xFF;
        let c = (version_hash >> 8) & 0xFF;
        let d = version_hash & 0xFF;
        let decrypted_version_hash = 0xFF ^ a ^ b ^ c ^ d;

        (version_from_header as u32) == decrypted_version_hash
    }

    // Test the version hash, then set the reader position back to its original position
    fn verify_version_hash(&mut self, version: i16, version_hash: u32) -> Result<(), Error> {
        let fallback_position = self.reader.as_mut().unwrap().get_position()?;
        let test_result = self.test_version_hash(version, version_hash);
        self.reader.as_mut().unwrap().seek(fallback_position)?;
        test_result
    }

    // Test the version hash using the reader
    // Update the hash for the reader here
    fn test_version_hash(&mut self, version: i16, version_hash: u32) -> Result<(), Error> {
        // Set the reader's hash
        self.reader.as_mut().unwrap().hash = version_hash;
        log::trace!("test: version {}, version_hash {}", version, version_hash);

        // Seek to the file offset for this version
        let offset = WzFile::get_offset_for_version(self.reader.as_ref().unwrap(), version);
        self.reader.as_mut().unwrap().seek(offset as u64)?;
        log::trace!("test: using offset {}", offset);

        // Create a new test directory
        let mut test_directory = WzDirectory {
            file: self,
            reader: self.reader.as_mut().unwrap(),
            offset,
            name: self.name.clone(),
            sub_directories: HashMap::new(),
            objects: HashMap::new(),
        };

        // Attempt to parse the root directory
        test_directory.parse_directory(false)?;
        if test_directory.sub_directories.is_empty() && test_directory.objects.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Failed directory test"));
        }

        // If there are objects, run additional tests
        if !test_directory.objects.is_empty() {
            let (_name, test_image) = match test_directory.objects.iter().next() {
                Some(v) => v,
                None => {
                    return Err(Error::new(ErrorKind::Other, "Failed to get next object"));
                }
            };

            self.reader
                .as_mut()
                .unwrap()
                .seek(test_image.offset.into())?;

            let test_byte = self.reader.as_mut().unwrap().read_u8()?;
            if test_byte != WzObject::HEADERBYTE_WITHOUT_OFFSET
                && test_byte != WzObject::HEADERBYTE_WITH_OFFSET
            {
                return Err(Error::new(ErrorKind::Other, "Failed byte test for object"));
            }
        }

        Ok(())
    }
}
