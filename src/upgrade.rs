use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// State for a single upgrade or research entry available to a player.
#[repr(C, packed)]
pub struct UpgradeInfo {
    /// Game-internal upgrade ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Currently researched level.
    pub current_level: u32,
    /// Maximum level this upgrade supports.
    pub max_level: u32,
    /// Progress towards the next level, as a percentage e.g. 25 -> 25%.
    pub upgrade_progress: u32,
    /// Path to the upgrade's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<UpgradeInfo>() == 216);
