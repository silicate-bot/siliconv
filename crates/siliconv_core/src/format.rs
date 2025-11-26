//! Module containing replay formats.

/// A replay format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    //
    // === GENERIC FORMATS === //
    //
    /// Generic plaintext format - intermediate representation.
    PlainText,

    //
    // === 2.1 FORMATS === //
    //
    /// ``OmegaBot`` .replay format.
    OmegaBot,
    /// ``OmegaBot`` 2 .replay format.
    OmegaBot2,
    /// ``OmegaBot`` 3 .replay format.
    OmegaBot3,

    /// Mega Hack Replay .json format.
    MHRJson,
    /// Mega Hack Replay .mhr format.
    MHR,

    /// ``TASBot`` .json format.
    TASBot,

    /// zBot .zbot format.
    ZBotXpos,
    /// zBot .zbf format.
    ZBotFrame,

    /// ``ReplayBot`` .replay format.
    ReplayBot,

    /// Echo old .echo format.
    EchoOld,
    /// Echo new .echo format, json.
    EchoNewJson,
    /// Echo new .echo format, binary.
    EchoNewBinary,

    /// yBot 1 no extension format.
    YBot1,

    /// xBot .xbot format.
    XBot,

    /// Rush .rsh format.
    Rush,

    /// ``KDBot`` .kd format.
    KDBot,

    //
    // === 2.2 FORMATS === //
    //
    /// GDR1 .gdr format.
    GDR1,
    /// GDR1 .gdr.json format.
    GDR1Json,
    /// GDR2 .gdr2 format.
    GDR2,
    /// xdBot legacy .xd format.
    XDBot,

    /// ``ReplayEngine`` v1 .re format.
    ReplayEngineV1,
    /// ``ReplayEngine`` v2 .re2 format.
    ReplayEngineV2,
    /// ``ReplayEngine`` v3 .re3 format.
    ReplayEngineV3,

    /// Silicate v1 .slc format.
    Slc1,
    /// Silicate v2 .slc format.
    Slc2,
    /// Silicate v3 .slc format.
    Slc3,

    /// ``UVBot`` .uv format.
    UVBot,
    /// ``TCBot`` .tcm format.
    TCBot,
}
