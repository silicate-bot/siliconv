const std = @import("std");

const Feature = @import("feature.zig").Feature;
const Version = @import("version.zig").Version;
const Input = @import("input.zig").Input;

pub const Format = enum {
    // cross-version
    plain_text,
    gdr1,
    gdr2,
    
    // 2.1
    xbot,
    ybot1,
    zbot,
    omegabot,
    omegabot2,
    omegabot3,
    mhr,
    mhr_json,
    tasbot,
    replaybot,
    echo_legacy,
    echo_json,
    echo_binary,
    rush,
    kdbot,
    rbot,
    
    // 2.2
    ybot2,
    ybot3,
    xdbot,
    re1,
    re2,
    re3,
    slc1,
    slc2,
    slc3,
    tcbot,
};

pub fn Replay(comptime T: type) type {
    return struct {
        features: Feature,
        orig_fmt: Format,
        version: Version,
        meta: T,

        inputs: std.ArrayList(Input)
    };
}
