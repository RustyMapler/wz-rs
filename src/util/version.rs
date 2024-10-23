use crate::{parse_directory, ArcWzNode, WzNode, WzReader, WzValueCast};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::Arc,
};

const WZ_GMS_OLD_IV: [u8; 4] = [0x4D, 0x23, 0xC7, 0x2B];
const WZ_GMS_IV: [u8; 4] = [0; 4];

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum WzVersion {
    GMS_OLD,
    GMS,
}

pub fn get_iv_for_version(version: WzVersion) -> [u8; 4] {
    match version {
        WzVersion::GMS_OLD => WZ_GMS_OLD_IV,
        WzVersion::GMS => WZ_GMS_IV,
    }
}

pub const INVALID_VERSION: i16 = -1;
const MAX_BRUTE_FORCE_VERSION: i16 = 230;

// Returns if this version is set to a valid value
fn is_version_valid(version: i16) -> bool {
    version != INVALID_VERSION
}

// Calculate the hash from version
fn calculate_version_hash(version: i16) -> u32 {
    let mut version_hash: u32 = 0;
    for c in version.to_string().chars() {
        version_hash = (32 * version_hash) + (c as u32) + 1;
    }

    version_hash
}

// Using the version hash, attempt to match the version from the file header
fn match_version_hash(version: i16, version_hash: u32) -> bool {
    let a = (version_hash >> 24) & 0xFF;
    let b = (version_hash >> 16) & 0xFF;
    let c = (version_hash >> 8) & 0xFF;
    let d = version_hash & 0xFF;
    let decrypted_version_hash = 0xFF ^ a ^ b ^ c ^ d;

    (version as u32) == decrypted_version_hash
}

// Test the version hash, then set the reader position back to its original position
fn verify_version_and_version_hash(
    reader: Arc<WzReader>,
    version: i16,
    version_hash: u32,
) -> Result<(), Error> {
    let original_position = reader.get_position()?;
    let test_result = test_version_and_version_hash(reader.clone(), version, version_hash);
    reader.seek(original_position)?;
    test_result
}

// Test the version and version hash with a dummy directory
fn test_version_and_version_hash(
    reader: Arc<WzReader>,
    version: i16,
    version_hash: u32,
) -> Result<(), Error> {
    // Set the reader's version hash
    reader.set_version_hash(version_hash);

    // Seek to the file offset for this version
    let file_start = *reader.file_start.borrow();
    let offset = get_version_offset(file_start as usize, version);
    reader.seek(offset as u64)?;

    // Test the root directory and look for other directories
    let node = parse_directory(&reader, offset, "Test Directory".to_string(), 0)?;
    let ref_node = node.as_ref();

    let directories: HashMap<String, Arc<WzNode>> = ref_node
        .children
        .clone()
        .into_iter()
        .filter(|(_k, v)| v.value.is_directory() || v.value.is_img())
        .collect();

    if directories.is_empty() {
        return Err(Error::new(ErrorKind::Other, "Failed directory test"));
    }

    let objects: HashMap<String, ArcWzNode> = node
        .children
        .clone()
        .into_iter()
        .filter(|(_k, v)| !v.value.is_directory() && !v.value.is_null())
        .collect();

    // If there are objects, check to see if it has the .img header
    if !objects.is_empty() {
        let object: &Arc<crate::WzNode> = match objects.iter().next() {
            Some((_, object)) => object,
            None => {
                return Err(Error::new(ErrorKind::Other, "Failed to get next object"));
            }
        };

        if object.value.is_null() {
            return Err(Error::new(ErrorKind::Other, "Failed object test"));
        }

        reader.seek(object.offset as u64)?;

        let test_byte = reader.read_u8()?;
        if test_byte != WzReader::HEADERBYTE_WITHOUT_OFFSET
            && test_byte != WzReader::HEADERBYTE_WITH_OFFSET
        {
            return Err(Error::new(ErrorKind::Other, "Failed byte test for object"));
        }
    }

    Ok(())
}

// For versions v230 or higher
fn detect_known_version(reader: Arc<WzReader>, version: u16) -> Result<bool, Error> {
    if version > 0xff {
        return Ok(true);
    } else if version == 0x80 {
        let file_start = *reader.file_start.borrow();
        reader.seek(file_start as u64)?;
        let property_count = reader.read_wz_int()?;
        if property_count > 0 && (property_count & 0xFF) == 0 && property_count <= 0xFFFF {
            return Ok(true);
        }
    }

    return Ok(false);
}

// Get the version by testing a known version
fn attempt_known_version(reader: Arc<WzReader>, version: i16) -> Option<(i16, u32)> {
    let version_hash = calculate_version_hash(version);
    match verify_version_and_version_hash(reader.clone(), version, version_hash) {
        Ok(_) => Some((version, version_hash)),
        Err(err) => {
            log::error!("attempt_known_version error: {}", err);
            None
        }
    }
}

// Get the version by testing all versions between 0 and MAX_BRUTE_FORCE_VERSION
fn bruteforce_version(reader: Arc<WzReader>, version: i16) -> Option<(i16, u32)> {
    for brute_force_version in 0..MAX_BRUTE_FORCE_VERSION {
        let brute_force_version_hash = calculate_version_hash(brute_force_version);
        if match_version_hash(version, brute_force_version_hash) {
            match verify_version_and_version_hash(
                reader.clone(),
                brute_force_version,
                brute_force_version_hash,
            ) {
                Ok(_) => return Some((brute_force_version, brute_force_version_hash)),
                Err(err) => {
                    log::error!("bruteforce_version error: {}", err);
                    continue;
                }
            }
        }
    }

    None
}

/// Parse the main directory for a .wz file. Nodes can only be resolved when parsed first.
pub fn determine_version(reader: Arc<WzReader>) -> Result<(i16, u32), Error> {
    let mut version: i16 = INVALID_VERSION;
    let mut version_hash: u32 = 0;

    // Determine file version
    let version_from_header = reader.read_u16()?;
    log::trace!("version from header: {}", version_from_header);

    let cloned_reader = reader.clone();

    // This is a known version, go ahead and test
    if detect_known_version(reader, version_from_header)? {
        const MAPLE_KNOWN_VERSION: i16 = 777;
        if let Some((attempt_version, attempt_hash)) =
            attempt_known_version(cloned_reader, MAPLE_KNOWN_VERSION)
        {
            version = attempt_version;
            version_hash = attempt_hash;
            log::trace!("success! patch version is v230 or greater!");
        } else {
            log::warn!("failed to use known patch version");
        }
    } else {
        // If we're using this in a custom client, we'll never have a patch version
        // Brute force the patch version instead
        if let Some((attempt_version, attempt_hash)) =
            bruteforce_version(cloned_reader, version_from_header as i16)
        {
            version = attempt_version;
            version_hash = attempt_hash;
            log::trace!("success! patch version is {}", attempt_version);
        } else {
            log::warn!("failed to bruteforce patch version");
        }
    }

    if !is_version_valid(version) {
        Err(Error::new(
            ErrorKind::NotFound,
            "Unable to determine version",
        ))
    } else {
        Ok((version, version_hash))
    }
}

// File offset depends on the version
pub fn get_version_offset(file_start: usize, version: i16) -> usize {
    if version > MAX_BRUTE_FORCE_VERSION {
        file_start
    } else {
        file_start + 2
    }
}
