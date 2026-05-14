use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// State for a single item held in a hero's inventory.
#[repr(C, packed)]
pub struct ItemInfo {
    /// Game-internal item ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Inventory slot index occupied by this item.
    pub slot: u32,
    /// Remaining charges, if the item is consumable.
    pub charges: u32,
    /// Path to the item's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ItemInfo>() == 212);
