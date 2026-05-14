use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 36;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// State for a single hero ability slot.
#[repr(C, packed)]
pub struct AbilityInfo {
    /// Game-internal ability ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Maximum cooldown of the ability, in seconds.
    pub cooldown: f32,
    /// Cooldown remaining before the ability can be used again, in seconds.
    pub cooldown_remaining: f32,
    /// Current learned level of the ability.
    pub level: u32,
    /// Path to the ability's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    /// Non-zero when the ability belongs to a hero.
    pub is_hero_ability: u8,
    /// Total damage dealt by uses of this ability.
    pub damage_dealt: u32,
    /// Total healing done by uses of this ability.
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<AbilityInfo>() == 161);
