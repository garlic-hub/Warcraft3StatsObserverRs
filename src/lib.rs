//! Rust bindings for the Warcraft 3 Stats Observer API memory map.
//!
//! Provides safe access to the shared-memory block exported by the Warcraft III
//! Stats Observer API. Use [`ObserverHandle`] as the entry point — it owns the
//! Windows file-mapping handles and dereferences to [`ObserverData`].
//!
//! # Example
//!
//! ```no_run
//! use warcraft3_stats_observer::ObserverHandle;
//!
//! let observer = ObserverHandle::new()
//!     .expect("Warcraft III not running or stats observer not available");
//!
//! println!("refresh rate: {} ms", { observer.refresh_rate });
//! ```
//!
//! # Packed structs
//!
//! All structs are `#[repr(C, packed)]` to mirror the layout of
//! the underlying memory map. Reading a misaligned field by reference is
//! undefined behaviour, so copy fields with `{}` before borrowing or
//! formatting them:
//!
//! ```ignore
//! println!("gold: {}", { player.gold });
//! ```

mod ability;
pub use ability::*;
mod build_queue;
pub use build_queue::*;
mod game;
pub use game::*;
mod hero;
pub use hero::*;
mod item;
pub use item::*;
mod observer;
pub use observer::*;
mod player;
pub use player::*;
mod player_item;
pub use player_item::*;
mod shop;
pub use shop::*;
mod shop_good;
pub use shop_good::*;
mod string_utils;
pub use string_utils::*;
mod structure;
pub use structure::*;
mod unit;
pub use unit::*;
mod upgrade;
pub use upgrade::*;
