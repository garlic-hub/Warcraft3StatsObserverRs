use std::ops::{Deref, DerefMut};
use winapi::shared::minwindef::LPVOID;
use winapi::shared::ntdef::HANDLE;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{FILE_MAP_WRITE, MapViewOfFile, OpenFileMappingW, UnmapViewOfFile};

use std::time::Duration;

use crate::game::ObserverGame;
use crate::player::PlayerInfo;
use crate::shop::ShopInfo;

const MAX_PLAYERS: usize = 28;
const MAX_SHOPS: usize = 999;

// Named tag where the warcraft 3 memory map lives
const OBSERVER_PATH: &str = r"War3StatsObserverSharedMemory";

#[repr(C, packed)]
pub struct ObserverData {
    pub version: u32,
    pub refresh_rate: u32,
    pub game: ObserverGame,
    pub players: [PlayerInfo; MAX_PLAYERS],
    pub shop_count: u32,
    pub shops: [ShopInfo; MAX_SHOPS],
}

impl ObserverData {
    pub fn disable(&mut self) {
        self.set_refresh_rate(Duration::ZERO);
    }

    pub fn set_refresh_rate(&mut self, duration: Duration) {
        self.refresh_rate = duration.as_millis() as u32;
    }
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ObserverData>() == 181219642);

/// Owns the Windows handles from `OpenFileMappingW` and `MapViewOfFile`.
/// Releases them via `Drop` and dereferences to `ObserverData`.
pub struct ObserverHandle {
    mapping: HANDLE,
    view: LPVOID,
}

unsafe impl Send for ObserverHandle {}
unsafe impl Sync for ObserverHandle {}

impl ObserverHandle {
    pub fn new() -> std::io::Result<Self> {
        Self::new_with_refresh_rate(Duration::from_millis(500))
    }

    pub fn new_with_refresh_rate(duration: Duration) -> std::io::Result<Self> {
        let mut path: Vec<u16> = OBSERVER_PATH.encode_utf16().collect();
        path.push(0);

        let mapping;
        let errno: i32;

        unsafe {
            mapping = OpenFileMappingW(FILE_MAP_WRITE, 0, path.as_ptr());
        }

        if mapping.is_null() {
            unsafe {
                errno = GetLastError() as i32;
            }
            return Err(std::io::Error::from_raw_os_error(errno));
        }

        let view: LPVOID;

        unsafe {
            view = MapViewOfFile(mapping, FILE_MAP_WRITE, 0, 0, 0);
        }

        if view.is_null() {
            unsafe {
                errno = GetLastError() as i32;
                CloseHandle(mapping);
            }
            return Err(std::io::Error::from_raw_os_error(errno));
        }

        let mut handle = ObserverHandle { mapping, view };
        handle.set_refresh_rate(duration);
        Ok(handle)
    }
}

impl Deref for ObserverHandle {
    type Target = ObserverData;

    fn deref(&self) -> &Self::Target {
        // SAFETY: view is non-null (checked at construction) and valid for the handle's lifetime.
        unsafe { &*(self.view as *const ObserverData) }
    }
}

impl DerefMut for ObserverHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: view is non-null (checked at construction), valid for the handle's lifetime,
        // and &mut self ensures no other mutable reference to this handle exists.
        unsafe { &mut *(self.view as *mut ObserverData) }
    }
}


impl Drop for ObserverHandle {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(self.view);
            CloseHandle(self.mapping);
        }
    }
}
