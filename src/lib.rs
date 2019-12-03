#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! ECSnap an Entity Component System designed to be packaged into UDP Packets for
//! Snapshot Interpolation of multiplayer games.
mod entity;
pub use entity::{Eid, Entity, EntityBuilder};

mod world;
pub use world::World;

mod system;
pub use system::{Read, System, SystemData, Write};
