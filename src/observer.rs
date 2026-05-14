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

/// Maximum number of player slots tracked by the observer API.
pub const MAX_PLAYERS: usize = 28;

/// Maximum number of shops tracked by the observer API.
pub const MAX_SHOPS: usize = 999;

// Named tag where the warcraft 3 memory map lives
const OBSERVER_PATH: &str = r"War3StatsObserverSharedMemory";

/// Top-level layout of the Warcraft III Stats Observer shared memory map.
///
/// This is the type [`ObserverHandle`] dereferences to. Because the underlying
/// memory is updated by Warcraft III, all numeric fields can change between
/// reads. The struct is `#[repr(C, packed)]`, so borrow individual fields by
/// copying them with `{ ... }`:
///
/// ```ignore
/// println!("version: {}", { observer.version });
/// ```
#[repr(C, packed)]
pub struct ObserverData {
    /// Not quite sure what this version number is supposed to represent.
    pub version: u32,
    /// Current refresh rate of the API in milliseconds. A value of `0` disables updates.
    pub refresh_rate: u32,
    /// Game-wide state (clock, map name, in-game flag, …).
    pub game: ObserverGame,
    /// Per-player state. Only the first
    /// [`ObserverGame::active_player_count`]
    /// entries are meaningful.
    pub players: [PlayerInfo; MAX_PLAYERS],
    /// Number of valid entries in [`Self::shops`].
    pub shop_count: u32,
    /// Per-shop state. Only the first [`Self::shop_count`]
    /// entries are meaningful.
    pub shops: [ShopInfo; MAX_SHOPS],
}

impl ObserverData {
    /// Disables observer updates by writing a refresh rate of `0`.
    pub fn disable(&mut self) {
        self.set_refresh_rate(Duration::ZERO);
    }

    /// Sets the observer's refresh rate. Sub-millisecond precision is
    /// truncated. A duration of zero disables updates.
    pub fn set_refresh_rate(&mut self, duration: Duration) {
        self.refresh_rate = duration.as_millis() as u32;
    }
}

// Number generated from SIZE fields of https://github.com/TinkerWorX/Blizzard.Net.Warcraft3
// noinspection RsAssertEqual
const _: () = assert!(size_of::<ObserverData>() == 181219642);

/// Owns the Windows handles from `OpenFileMappingW` and `MapViewOfFile`.
///
/// Releases them via `Drop` and dereferences to [`ObserverData`] for read and
/// write access to the memory map.
pub struct ObserverHandle {
    mapping: HANDLE,
    view: LPVOID,
}

// SAFETY:
// `Send` / `Sync` are required because the raw Win32 handle and pointer fields
// make `ObserverHandle` opt out of these traits by default.
//
// - `Send` is fine: the file mapping and view belong to the process, not to a
//   specific thread. Any thread may use them and any thread may free them
//   (which `Drop` does).
// - `Sync` is fine: the only shared-reference operation is reading
//   `ObserverData` via `Deref`. Mutation goes through `&mut self`, so Rust's
//   borrow rules already prevent reader/writer races between threads of *this*
//   program.
//
// What these impls do NOT promise: Warcraft III is mutating the same memory
// from another process. We can't synchronize with it, so callers should treat
// each field read as a possibly-stale snapshot rather than a coherent view.
unsafe impl Send for ObserverHandle {}
unsafe impl Sync for ObserverHandle {}

impl ObserverHandle {
    /// Opens the observer memory map and sets the refresh rate to the default
    /// of 500 ms.
    ///
    /// Returns an error if Warcraft III is not running, or the stats observer
    /// API is otherwise unavailable.
    pub fn new() -> std::io::Result<Self> {
        Self::new_with_refresh_rate(Duration::from_millis(500))
    }

    /// Opens the observer memory map and writes `duration` as the refresh
    /// rate. A duration of zero disables updates.
    ///
    /// Returns an error if Warcraft III is not running, or the stats observer
    /// API is otherwise unavailable.
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
