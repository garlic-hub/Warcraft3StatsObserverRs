use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// Aggregate state for one unit kind owned by a player.
#[repr(C, packed)]
pub struct UnitInfo {
    /// Game-internal unit ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// ID of the owning player.
    pub owner_id: u32,
    /// Number currently alive.
    pub current_amount: u32,
    /// Number ever trained / spawned.
    pub total_amount: u32,
    /// Path to the unit's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    /// Whether the unit is classified as a worker.
    /// Not sure how this differs from `is_function_peop`[Self::is_functional_peon].
    pub is_peon: bool,
    /// Whether the unit is classified as a worker.
    /// Not sure how this differs from `is_peon`[Self::is_peon].
    pub is_functional_peon: bool,
    /// Total damage dealt by units of this kind.
    pub damage_dealt: u32,
    /// Total damage received by units of this kind.
    pub damage_received: u32,
    /// Total healing done by units of this kind.
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<UnitInfo>() == 230);
