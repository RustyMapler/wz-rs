#[cfg(test)]
mod wzfile_tests {
    use crate::{wz_file::WzFile, WzVersion};
    use std::io::{Error, ErrorKind};

    #[test]
    fn wzfile_open_invalid_file() {
        let _expected = Error::new(ErrorKind::Other, "Invalid .wz file");

        let filename = "invalid_file";
        let mut file = WzFile::new(filename, WzVersion::GMS);
        let result = file.open().err();

        assert!(matches!(result, _expected));
    }
}
