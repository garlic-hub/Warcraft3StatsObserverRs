use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// State for a single structure owned by a player.
#[repr(C, packed)]
pub struct StructureInfo {
    /// Game-internal structure ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Build progress towards completion, as a percentage e.g. 25 -> 25%
    pub construction_progress: u32,
    /// Build progress of an upgrade affecting this structure, as a percentage
    /// e.g. Town Hall -> Keep
    pub upgrade_progress: u32,
    /// Path to the structure's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<StructureInfo>() == 212);
