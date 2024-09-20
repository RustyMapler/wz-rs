const WZ_GMS_OLD_IV: [u8; 4] = [0x4D, 0x23, 0xC7, 0x2B];
const WZ_GMS_IV: [u8; 4] = [0; 4];

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
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
