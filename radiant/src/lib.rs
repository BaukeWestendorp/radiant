//! `radiant` is the backend for any Radiant frontend.
//!
//! This crate provides the core logic, data structures, and engine for the
//! Radiant lighting control system. It is frontend-agnostic and designed to be
//! embedded in various applications, including GUIs, CLIs, and automated
//! systems.
//!
//! The core communicates through a flexible [Command][crate::cmd::Command]
//! system, allowing external interfaces to control and modify the show state,
//! patch, and output in a decoupled manner. The engine manages the lifecycle of
//! a show, processes commands, and produces DMX output for connected devices.

#![warn(missing_docs)]

pub mod cmd;
pub mod engine;
pub mod error;
pub mod object;
pub mod patch;
pub mod show;
pub mod showfile;

mod adapters;
mod pipeline;
mod protocols;

/// Re-export of `gdtf` crate.
pub mod gdtf {
    pub use gdtf::*;
}
