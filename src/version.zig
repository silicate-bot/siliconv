
/// The version of the replay.
///
/// # Examples
/// ```zig
/// // 2.206
/// const version = Version {
///     .major = 22,
///     .minor = 60,
/// };
///
/// // 2.21
/// const version = Version {
///     .major = 22,
///     .minor = 100,
/// };
///
/// // 2.2074
/// const version = Version {
///     .major = 22,
///     .minor = 74,
/// };
/// ```
pub const Version = struct {
    major: u32,
    minor: u32,
    
    fn equal(self: *Version, other: *Version) bool {
        return self.major == other.major;
    }
    
    // 2.2074
    pub const latest = Version {
        .major = 22,
        .minor = 74,
    };
};
