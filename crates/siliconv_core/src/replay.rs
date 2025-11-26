//! Module for the replay struct.

use crate::{action::TimedAction, format::Format, meta::Meta, version::GameVersion};

/// A replay.
pub struct Replay {
    /// Metadata related to the replay.
    pub meta: Box<dyn Meta>,
    /// Replay actions.
    pub actions: Vec<TimedAction>,
    /// The initial replay format for this replay.
    pub format: Format,
    /// The game version this replay is designed for.
    pub game_version: GameVersion,
}
