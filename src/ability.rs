use crate::string_utils::PaddedString;

const MAX_NAME_LENGTH: usize = 36;
const MAX_BUTTON_ART_LENGTH: usize = 100;

#[repr(C, packed)]
pub struct AbilityInfo {
    pub id: u32,
    pub name: PaddedString<MAX_NAME_LENGTH>,
    pub cooldown: f32,
    pub cooldown_remaining: f32,
    pub level: u32,
    pub button_art: PaddedString<MAX_BUTTON_ART_LENGTH>,
    pub is_hero_ability: u8,
    pub damage_dealt: u32,
    pub healing_done: u32,
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<AbilityInfo>() == 161);
