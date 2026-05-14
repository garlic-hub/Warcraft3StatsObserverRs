use std::time::Duration;

use crate::string_utils::PaddedString;

const MAX_GAME_NAME_LENGTH: usize = 256;
const MAX_MAP_NAME_LENGTH: usize = 256;

/// Game-wide state exposed by the Warcraft III Stats Observer API.
#[repr(C, packed)]
pub struct ObserverGame {
    /// `true` while a game is in progress.
    pub in_game: bool,
    /// Elapsed game time in milliseconds. Use [`Self::time`] for a
    /// strongly-typed [`Duration`].
    pub clock_ms: u32,
    /// Number of valid entries in [`crate::ObserverData::players`].
    pub active_player_count: u8,
    /// Name of the current game / lobby.
    pub game_name: PaddedString<MAX_GAME_NAME_LENGTH>,
    /// Name of the map currently being played.
    pub map_name: PaddedString<MAX_MAP_NAME_LENGTH>,
}

impl ObserverGame {
    /// Returns the elapsed game time as a [`Duration`].
    pub fn time(&self) -> Duration {
        Duration::from_millis(self.clock_ms as u64)
    }
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ObserverGame>() == 518);
