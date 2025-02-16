// In cases of formats with multiple versions (TCBot, Silicate, RE)
// the version should automatically be parsed based on the meta/structure of the file.
//
// GDR2 is explicitly a different version due to it having been physically separated from GDR1.
// This is the same for OmegaBot replays.

/// The original format of the currently loaded replay.
///
#[derive(Clone, PartialEq)]
pub enum ReplayFormat {
    // Cross-version
    PlainText,
    GDR1, // In theory GDR is version-agnostic, game version will be parsed from meta
    GDR2,

    // 2.1 formats
    XBot,
    YBot1,
    ZBot,
    OmegaBot, // also known as URL
    OmegaBot2,
    OmegaBot3,
    MHR,
    MHRJson,
    TASBot,
    ReplayBot,
    EchoLegacy,
    EchoJson,
    EchoBinary,
    Rush,
    KDBot,
    RBot,

    // 2.2 formats
    YBot2,
    XDBot,
    ReplayEngine,
    Silicate,
    TCBot,
}
