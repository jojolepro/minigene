pub extern crate bracket_lib;
pub extern crate game_features;
pub extern crate hibitset;

#[cfg(feature = "terminal")]
extern crate crossterm;

pub use bracket_lib::prelude::{
    a_star_search, add_wasm_support, field_of_view, main_loop, to_cp437, Algorithm2D, BError,
    BEvent, BTerm, BTermBuilder, BaseMap, GameState, MultiTileSprite, NavigationPath, Point, Rect,
    SmallVec, SpriteSheet, VirtualKeyCode, BLACK, BLUE, EMBED, GREEN, INPUT, RED, RGBA, WHITE,
    YELLOW,
};
pub use game_clock::*;
pub use game_features::*;
pub use hibitset::BitSet as HBitSet;
pub use stopwatch::*;

pub use minigene_core::*;


// macro re-export
pub use derive_new::*;

pub use spin_sleep::LoopHelper;

mod components;
mod macros;
mod render;
mod resources;
mod systems;
mod utils;

pub use self::components::*;
pub use self::macros::*;
pub use self::render::*;
pub use self::resources::*;
pub use self::systems::*;
pub use self::utils::*;

use std::collections::HashMap;

use std::fmt::Debug;
use std::hash::Hash;

