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
//!
//! # Example
//! ```
//! extern crate ecsnap;
//! use ecsnap::{Component, System, World};
//!
//! // Components are structs which hold data for an Entity. Components must derive
//! // Debug, Clone, & Copy.
//! #[derive(Debug, Clone, Copy)]
//! struct Pos {
//!     x: f64,
//!     y: f64,
//! }
//!
//! #[derive(Debug, Clone, Copy)]
//! struct Vel {
//!     x: f64,
//!     y: f64,
//! }
//!
//! // Todo: make Component Derivable.
//! impl Component for Pos {}
//! impl Component for Vel {}
//!
//! // Systems are structs which can have internal data & operate on Components
//! struct MovementSystem {
//!     dt: f64,         
//! }
//!
//! // Implementing System
//! impl System for MovementSystem {
//!     // Define the components required for this system.    
//!     type Data = (Pos, Vel);
//!     // Define the operation on the Component data. All data fetched is mutable.
//!     fn run(&mut self, data: &mut Self::Data){
//!         let (pos, vel) = data;
//!         pos.x += vel.x * self.dt;
//!         pos.y += vel.y * self.dt;
//!         println!("Updated Position! {:?}", pos);
//!     }
//! }
//!
//! let mut mvt = MovementSystem { dt : 0.05 };
//!
//! // Create the world.
//! let mut world = World::default();
//!
//! // Add an Entity with Pos and Vel components to the World. Store the specific
//! // EntityID (Eid) in _e1.
//! let _e1 = world
//!     .create_entity()
//!     .with(Pos {x: 0.0, y: 0.0})
//!     .with(Vel {x: 10.0, y: 10.0})
//!     .build();
//!
//! // Add an Entity just with Pos component to the World.
//! world
//!     .create_entity()
//!     .with(Pos {x: 0.0, y: 0.0})
//!     .build();
//!
//! // Dispatch the system twice.
//! world.dispatch_system(&mut mvt);
//! world.dispatch_system(&mut mvt);
//! ```
mod component;
pub use component::Component;

mod entity;
pub use entity::{Eid, Entity, EntityBuilder};

mod world;
pub use world::World;

mod system;
pub use system::{System, SystemData};
