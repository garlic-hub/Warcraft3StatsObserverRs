use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

/// State for a single item offered for sale at a shop.
#[repr(C, packed)]
pub struct ShopGoodInfo {
    /// Game-internal item ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Amount of item in stock.
    pub stock: u32,
    /// Maximum units that can be stocked.
    pub max_stock: u32,
    /// Restock cooldown, in seconds.
    pub cooldown: f32,
    /// Time remaining before the next restock, in seconds.
    pub cooldown_remaining: f32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ShopGoodInfo>() == 120);
