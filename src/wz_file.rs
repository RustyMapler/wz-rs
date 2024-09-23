use crate::{
    determine_version, get_iv_for_version, get_version_offset, resolve_uol_path,
    wz_crypto::generate_wz_key, WzDirectory, WzNode, WzReader, WzVersion, INVALID_VERSION,
};
use std::{
    cell::RefCell,
    collections::HashMap,
    fs::{self, File},
    io::{Cursor, Error, ErrorKind, Read},
    path::Path,
    sync::Arc,
};

pub struct WzFile {
    pub name: String,
    pub reader: Arc<WzReader>,
    pub version: i16,
    pub version_hash: u32,
    pub root: Option<WzDirectory>,
    pub file_path: String,
    pub wz_version: WzVersion,
}

impl WzFile {
    pub fn new(path: &str, version: WzVersion) -> WzFile {
        let file_path = Path::new(path);

        WzFile {
            name: file_path.file_name().unwrap().to_str().unwrap().into(),
            file_path: path.to_string(),
            reader: Arc::new(WzReader {
                file: RefCell::new(Cursor::new(Vec::new())),
                wz_key: None,
                file_start: 0,
                version_hash: 0.into(),
            }),
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

        // Create reader
        let mut reader = WzReader {
            file: Cursor::new(buffer).into(),
            wz_key: generate_wz_key(get_iv_for_version(self.wz_version)),
            file_start: 0,
            version_hash: 0.into(),
        };
        reader.file_start = WzFile::parse_wz_header(&reader)?;

        if let Ok((version, version_hash)) = determine_version(reader.clone().into()) {
            self.version = version;
            self.version_hash = version_hash;
            reader.set_version_hash(version_hash);
        }

        self.reader = reader.into();

        self.parse_wz_main_directory()?;

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

    /// Parse the header for a .wz file. Get the file start for the reader.
    fn parse_wz_header(reader: &WzReader) -> Result<u32, Error> {
        let ident = reader.read_string(4)?;
        log::trace!("ident: {}", ident);

        if ident != "PKG1" {
            return Err(Error::new(ErrorKind::Other, "Invalid .wz file"));
        }

        let size = reader.read_u64()?;
        let start = reader.read_u32()?;
        let copyright = reader.read_string_to_end()?;

        log::trace!("size: {}, start: {}, copyright {}", size, start, copyright);

        Ok(start)
    }

    /// Parse the main directory for a .wz file. Nodes can only be resolved when parsed first.
    fn parse_wz_main_directory(&mut self) -> Result<(), Error> {
        let offset = get_version_offset(self.reader.file_start, self.version);

        self.root = Some(WzDirectory {
            reader: self.reader.clone(),
            offset,
            name: self.name.clone(),
            directories: HashMap::new(),
            objects: HashMap::new(),
        });
        self.root.as_mut().unwrap().parse_directory(true).unwrap();

        Ok(())
    }
}
