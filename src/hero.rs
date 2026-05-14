use crate::ability::AbilityInfo;
use crate::item::ItemInfo;
use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;
const MAX_ABILITIES: usize = 24;
const MAX_ITEMS: usize = 6;

/// State for a single hero owned by a player.
#[repr(C, packed)]
pub struct HeroInfo {
    /// Game-internal hero ID.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Path to the hero's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    /// Current hero level.
    pub level: u32,
    /// Total experience accumulated.
    pub experience: u32,
    /// Experience required to reach the next level.
    pub level_up_experience: u32,
    /// Current hit points.
    pub hit_points: u32,
    /// Maximum hit points.
    pub max_hit_points: u32,
    /// Current mana points.
    pub mana_points: u32,
    /// Maximum mana points.
    pub max_mana_points: u32,
    /// Total damage this hero has dealt.
    pub damage_dealt: u32,
    /// Total damage this hero has received.
    pub damage_received: u32,
    /// Damage dealt by this hero to itself.
    pub self_damage: u32,
    /// Whether this hero was picked 1st, 2nd, 3rd, etc.
    pub pick_order: u32,
    /// Total healing done by this hero.
    pub healing_done: u32,
    /// Number of times this hero has died.
    pub number_of_deaths: u32,
    /// Total kills credited to this hero.
    pub total_kills: u32,
    /// Self-inflicted kills.
    pub self_kills: u32,
    /// Enemy heroes killed.
    pub hero_kills: u32,
    /// Enemy buildings destroyed.
    pub building_kills: u32,
    /// Total time this hero has been alive, in milliseconds.
    pub time_alive_ms: u32,
    /// Number of valid entries in [`Self::ability_info`].
    pub ability_count: u32,
    /// Hero abilities (and any non-hero abilities the unit also has).
    pub ability_info: [AbilityInfo; MAX_ABILITIES],
    /// Number of valid entries in [`Self::items`].
    pub item_count: u32,
    /// Items currently in the hero's inventory.
    pub items: [ItemInfo; MAX_ITEMS],
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<HeroInfo>() == 5420);
