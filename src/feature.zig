const std = @import("std");

pub const Feature = packed struct(u32) {
    skip_input: bool = false,

    tps_change: bool = false,

    death: bool = false,
    restart: bool = false,
    restart_full: bool = false,
    bug_checkpoint: bool = false,

    frame_fixes: bool = false,

    set_seed: bool = false,
    seed_per_attempt: bool = false,

    padding: u23 = 0,

    comptime {
        std.debug.assert(@sizeOf(Feature) == 4);
    }
};
