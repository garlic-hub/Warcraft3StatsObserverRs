use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

/// Aggregate per-player statistics for a specific item type.
#[repr(C, packed)]
pub struct PlayerItemInfo {
    /// Game-internal item ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Item tier / level.
    pub item_level: u32,
    /// Total number of this item the player has obtained from any source.
    pub collected: u32,
    /// Number purchased from a shop.
    pub purchased: u32,
    /// Number sold back to a shop.
    pub sold: u32,
    /// Number consumed by use.
    pub used: u32,
    /// Number destroyed.
    pub destroyed: u32,
    /// Total damage dealt by this item.
    pub damaged_dealt: u32,
    /// Total healing done by this item.
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<PlayerItemInfo>() == 136);
