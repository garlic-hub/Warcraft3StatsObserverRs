use std::fmt::Debug;

use crate::build_queue::BuildQueueInfo;
use crate::hero::HeroInfo;
use crate::player_item::PlayerItemInfo;
use crate::string_utils::PaddedString;
use crate::structure::StructureInfo;
use crate::unit::UnitInfo;
use crate::upgrade::UpgradeInfo;

const MAX_NAME_LENGTH: usize = 36;

/// Maximum number of heroes per player tracked by the observer API.
pub const MAX_HEROES: usize = 999;
/// Maximum number of structures per player tracked by the observer API.
pub const MAX_STRUCTURES: usize = 999;
/// Maximum number of upgrades per player tracked by the observer API.
pub const MAX_UPGRADES: usize = 999;
/// Maximum number of unit kinds per player tracked by the observer API.
pub const MAX_UNITS: usize = 999;
/// Maximum number of in-progress build queue entries per player.
pub const MAX_UNITS_IN_QUEUE: usize = 999;
/// Maximum number of distinct items collected per player.
pub const MAX_ITEMS: usize = 999;
/// Number of upkeep levels reported by the observer API.
pub const MAX_UPKEEP_LEVELS: usize = 10;

/// Race chosen by the player in the lobby. Bit-flag encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RacePreference {
    Human = 0x01,
    Orc = 0x02,
    Nightelf = 0x04,
    Undead = 0x08,
    Demon = 0x10,
    Random = 0x20,
    UserSelectable = 0x40,
}

/// Race a player is actually playing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlayerRace {
    Unknown = 0,
    Human = 1,
    Orc = 2,
    Undead = 3,
    NightElf = 4,
    Demon = 5,
    Last = 6,
    Other = 7,
    Creep = 8,
    Commoner = 9,
    Critter = 10,
    Naga = 11,
}

/// What occupies a player slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlayerType {
    Empty = 0,
    Player = 1,
    Computer = 2,
    Neutral = 3,
    Observer = 4,
    None = 5,
    Other = 6,
}

/// End-of-game result for a player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlayerGameResult {
    Victory = 0,
    Defeat = 1,
    Tie = 2,
    Neutral = 3,
}

/// Live state of a player slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlayerSlotState {
    Empty = 0,
    Playing = 1,
    Left = 2,
}

/// Difficulty preference selected for a computer player.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AiDifficultyPreference {
    Newbie = 0,
    Normal = 1,
    Insane = 2,
}

/// Per-player state — name, race, resources, and arrays of heroes,
/// structures, units, upgrades, items, and build queue entries.
#[repr(C, packed)]
pub struct PlayerInfo {
    /// Display name of the player.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Race the player chose in the lobby (may be `Random` or
    /// `UserSelectable`).
    pub race_preference: RacePreference,
    /// Race the player is actually playing this game.
    pub player_race: PlayerRace,
    /// Game-internal player ID (slot number).
    pub id: u8,
    /// Index of the team the player belongs to.
    pub team_index: u8,
    /// Team color index assigned to the player.
    pub team_color: u8,
    /// What kind of participant occupies this slot (human, computer, observer, …).
    pub player_type: PlayerType,
    /// Handicap percentage (e.g. `100` = no handicap, `50` = 50% max HP).
    pub handicap: u32,
    /// End-of-game result, once the game has ended.
    pub game_result: PlayerGameResult,
    /// Live state of the player's slot (empty / playing / left).
    pub slot_state: PlayerSlotState,
    /// Difficulty selected for this player when [`Self::player_type`]
    /// is `Computer`.
    pub ai_difficulty: AiDifficultyPreference,
    /// Some sort of APM measurement, now sure how it differs from
    /// [`Self::real_time_actions_per_minute`].
    pub actions_per_minute: u32,
    /// Some sort of APM measurement, now sure how it differs from
    /// [`Self::actions_per_minute`].
    pub real_time_actions_per_minute: u32,
    /// Current gold on hand.
    pub gold: u32,
    /// Total gold mined this game.
    pub gold_mined: u32,
    /// Gold lost to upkeep over the course of the game.
    pub gold_upkeep_lost: u32,
    /// Not sure what this is
    pub gold_diversion_tax: u32,
    /// Current lumber on hand.
    pub lumber: u32,
    /// Total lumber harvested this game.
    pub lumber_mined: u32,
    /// Lumber lost to upkeep over the course of the game.
    pub lumber_upkeep_lost: u32,
    /// Not sure what this is.
    pub lumber_diversion_tax: u32,
    /// Current food cap (maximum population).
    pub food_cap: u32,
    /// Current food used (current population).
    pub food_used: u32,
    /// Number of valid entries in [`Self::heroes`].
    pub hero_count: u32,
    /// Heroes owned by the player.
    pub heroes: [HeroInfo; MAX_HEROES],
    /// Number of valid entries in [`Self::structures`].
    pub structure_count: u32,
    /// Structures owned by the player.
    pub structures: [StructureInfo; MAX_STRUCTURES],
    /// Number of valid entries in [`Self::upgrades`].
    pub upgrade_count: u32,
    /// Upgrades available to (researched or in progress by) the player.
    pub upgrades: [UpgradeInfo; MAX_UPGRADES],
    /// Number of valid entries in [`Self::units`].
    pub unit_count: u32,
    /// Aggregate stats per unit kind owned by the player.
    pub units: [UnitInfo; MAX_UNITS],
    /// Number of valid entries in [`Self::units_in_queue`].
    pub units_in_queue_count: u32,
    /// In-progress training, research, and revival entries.
    pub units_in_queue: [BuildQueueInfo; MAX_UNITS_IN_QUEUE],
    /// Number of valid entries in [`Self::items`].
    pub item_count: u32,
    /// Aggregate stats per item type the player has interacted with.
    pub items: [PlayerItemInfo; MAX_ITEMS],
    /// Time spent in each upkeep level, indexed by level (in milliseconds).
    /// Not reliable for replays because it looks at wall time rather than in-game time
    /// i.e. replays can be paused, slow-down, or sped up, but it doesn't account for that.
    /// Unknown if it keeps ticking while the "waiting for players" screen is counting down.
    pub time_in_upkeep: [u32; MAX_UPKEEP_LEVELS],
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<PlayerInfo>() == 6416738);
