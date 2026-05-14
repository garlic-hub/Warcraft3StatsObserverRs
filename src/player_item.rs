use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct PlayerItemInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub item_level: u32,
    pub collected: u32,
    pub purchased: u32,
    pub sold: u32,
    pub used: u32,
    pub destroyed: u32,
    pub damaged_dealt: u32,
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<PlayerItemInfo>() == 136);
