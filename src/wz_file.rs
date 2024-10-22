use crate::{
    determine_version, get_iv_for_version, get_version_offset, parse_directory,
    wz_crypto::generate_wz_key, ArcWzNode, WzReader, WzVersion, INVALID_VERSION,
};
use std::{
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
    pub file_path: String,
    pub wz_version: WzVersion,
}

impl WzFile {
    pub fn new(path: &str, version: WzVersion) -> WzFile {
        let file_path = Path::new(path);

        WzFile {
            name: file_path.file_name().unwrap().to_str().unwrap().into(),
            file_path: path.to_string(),
            reader: Arc::new(WzReader::new(Cursor::new(Vec::new()), None)),
            version: INVALID_VERSION,
            version_hash: 0,
            wz_version: version,
        }
    }

    /// Creates a WzFile from filepath
    pub fn open(&mut self) -> Result<(), Error> {
        let file_path = Path::new(&self.file_path);

        let mut file = File::open(file_path)?;
        let metadata = fs::metadata(file_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)?;

        // Create reader
        let mut reader = WzReader::new(
            Cursor::new(buffer),
            generate_wz_key(get_iv_for_version(self.wz_version)),
        );

        reader.file_start = WzFile::parse_wz_header(&reader)?.into();

        if let Ok((version, version_hash)) = determine_version(reader.clone().into()) {
            self.version = version;
            self.version_hash = version_hash;
            reader.set_version_hash(version_hash);
        }

        self.reader = reader.into();

        Ok(())
    }

    /// Parse the header for a .wz file. Get the file start for the reader.
    fn parse_wz_header(reader: &WzReader) -> Result<u32, Error> {
        let ident = reader.read_string(4)?;

        if ident != "PKG1" {
            return Err(Error::new(ErrorKind::Other, "Invalid .wz file"));
        }

        let _size = reader.read_u64()?;
        let start = reader.read_u32()?;
        let _copyright = reader.read_string_to_end()?;

        Ok(start)
    }

    pub fn parse_root_directory(&mut self) -> Result<ArcWzNode, Error> {
        let offset = get_version_offset(*self.reader.file_start.borrow() as usize, self.version);

        let node = parse_directory(self.name.clone(), &self.reader.clone(), offset)?;

        Ok(node)
    }
}
