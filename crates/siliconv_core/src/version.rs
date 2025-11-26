//! Game version related structs.

/// A game version.
pub struct GameVersion {
    /// The major version number.
    ///
    /// Given version "1.9" this would be `19`.
    /// Given version "2.113" this would be `21`.
    /// Given version "2.2074" this would be `22`.
    pub major: u32,

    /// The minor version number.
    ///
    /// This number has trailing zeroes added
    /// until the version reaches "4-digit precision".
    ///
    /// Given version "1.9" this would be `0`.
    /// Given version "2.113" this would be `130`.
    /// Given version "2.2074" this would be `74`.
    pub minor: u32,
}

impl GameVersion {
    /// Check if this version is universal (major version 0).
    #[must_use]
    pub const fn is_universal(&self) -> bool {
        self.major == 0
    }

    /// Create a universal game version.
    #[must_use]
    pub fn universal() -> Self {
        Self { major: 0, minor: 0 }
    }
}

impl PartialEq for GameVersion {
    fn eq(&self, other: &Self) -> bool {
        self.is_universal() || other.is_universal() || self.major == other.major
    }
}
