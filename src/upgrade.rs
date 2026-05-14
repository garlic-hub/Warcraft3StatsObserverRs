use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct UpgradeInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub current_level: u32,
    pub max_level: u32,
    pub upgrade_progress: u32,
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<UpgradeInfo>() == 216);
