//! Module for the replay struct.

use std::io::{Read, Seek, Write};

use crate::{
    action::TimedAction, error::ReplayError, format::Format, meta::Meta, version::GameVersion,
};

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

/// Trait for types that are:
/// - Convertible to and from a generic replay.
/// - Serializable to and from a reader/writer.
pub trait ReplaySerializable {
    /// Create a new instance from a generic replay.
    fn new(replay: Replay) -> Self;

    /// Downcast into a generic replay.
    fn into_replay(self) -> Replay;

    /// Read an instance from a reader.
    ///
    /// # Errors
    /// If reading from the reader fails.
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, ReplayError>
    where
        Self: Sized;

    /// Write an instance to a writer.
    ///
    /// # Errors
    /// If writing to the writer fails.
    fn write<W: Write>(&self, writer: &mut W) -> Result<(), ReplayError>;
}
