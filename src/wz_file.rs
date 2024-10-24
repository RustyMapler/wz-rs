use crate::{
    crypto::generate_wz_key, determine_version, get_iv_for_version, get_version_offset,
    parse_directory, parse_wz_header, ArcWzNode, WzReader, WzVersion, INVALID_VERSION,
};
use std::{
    fs::{self, File},
    io::{Cursor, Error, Read},
    path::PathBuf,
    sync::Arc,
};

pub struct WzFile {
    pub file_path: PathBuf,
    pub file_version: WzVersion,
    pub name: String,
    pub reader: Arc<WzReader>,
    pub version: i16,
    pub version_hash: u32,
}

impl WzFile {
    pub fn new(path: &str, version: WzVersion) -> Result<WzFile, Error> {
        let file_path = PathBuf::from(path);

        let name = file_path
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .ok_or_else(|| Error::new(std::io::ErrorKind::InvalidInput, "Invalid file name"))?
            .into();

        Ok(WzFile {
            file_path,
            file_version: version,
            name,
            reader: Arc::default(),
            version: INVALID_VERSION,
            version_hash: 0,
        })
    }

    pub fn open(&mut self) -> Result<(), Error> {
        let file_path = &self.file_path;
        let mut file = File::open(file_path)?;
        let metadata = fs::metadata(file_path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        file.read_exact(&mut buffer)?;

        let mut reader = WzReader::new(
            Cursor::new(buffer),
            generate_wz_key(get_iv_for_version(self.file_version)),
        );

        reader.file_start = parse_wz_header(&reader)?.into();

        self.determine_and_set_version(&mut reader);

        self.reader = reader.into();

        Ok(())
    }

    pub fn parse_root_directory(&mut self) -> Result<ArcWzNode, Error> {
        let offset = get_version_offset(*self.reader.file_start.borrow() as usize, self.version);
        let level = 99;

        let node = parse_directory(&self.reader.clone(), offset, self.name.clone(), level)?;

        Ok(node)
    }

    fn determine_and_set_version(&mut self, reader: &mut WzReader) {
        let mut try_set_version = |wz_version| {
            reader.set_wz_key(generate_wz_key(get_iv_for_version(wz_version)));
            if let Ok((version, version_hash)) = determine_version(reader.clone().into()) {
                self.version = version;
                self.version_hash = version_hash;
                reader.set_version_hash(version_hash);
                return true;
            }
            false
        };

        // Try to determine version with the current or auto-detected IV
        if self.file_version == WzVersion::AUTO_DETECT {
            if try_set_version(self.file_version) {
                return;
            }
            // If auto-detect fails, try with GMS_OLD's IV
            try_set_version(WzVersion::GMS_OLD);
        } else {
            try_set_version(self.file_version);
        }
    }
}
