use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 100;
const MAX_BUTTON_ART_LENGTH: usize = 100;

/// What kind of work a build queue entry represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BuildQueueType {
    /// An upgrade or technology being researched.
    Research = 0,
    /// A unit being trained.
    Unit = 1,
    /// A hero being revived.
    Reviving = 2,
}

/// State for a single entry in a player's build queue.
#[repr(C, packed)]
pub struct BuildQueueInfo {
    /// Game-internal ID of the thing being trained, researched, or revived.
    pub id: u32,
    /// Display name.
    pub name: PaddedString<MAX_NAME_LENGTH>,
    /// Progress towards completion (game-defined units).
    pub training_progress: u32,
    /// Whether this entry is a unit, an upgrade, or a hero revival.
    pub build_type: BuildQueueType,
    /// Path to the queue entry's button icon.
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<BuildQueueInfo>() == 209);
