use crate::shop_good::ShopGoodInfo;
use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;

/// Maximum number of goods sold by a single shop.
pub const MAX_GOODS: usize = 12;

/// State for a single shop available on the map.
#[repr(C, packed)]
pub struct ShopInfo {
    /// Game-internal shop ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Not sure what this is.
    pub owner_id: u32,
    /// Number of valid entries in [`Self::goods`].
    pub goods_count: u32,
    /// Goods offered by the shop.
    pub goods: [ShopGoodInfo; MAX_GOODS],
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ShopInfo>() == 1552);
