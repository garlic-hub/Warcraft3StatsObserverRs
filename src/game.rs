use std::time::Duration;

use crate::string_utils::PaddedString;

const MAX_GAME_NAME_LENGTH: usize = 256;
const MAX_MAP_NAME_LENGTH: usize = 256;

#[repr(C, packed)]
pub struct ObserverGame {
    pub in_game: bool,
    pub clock_ms: u32,
    pub active_player_count: u8,
    pub game_name: PaddedString<MAX_GAME_NAME_LENGTH>,
    pub map_name: PaddedString<MAX_MAP_NAME_LENGTH>,
}

impl ObserverGame {
    pub fn time(&self) -> Duration {
        Duration::from_millis(self.clock_ms as u64)
    }
}
// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ObserverGame>() == 518);
