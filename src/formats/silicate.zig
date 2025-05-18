const std = @import("std");
const assert = std.debug.assert;
const Replay = @import("../replay.zig").Replay;
const Feature = @import("../feature.zig").Feature;
const inputMod = @import("../input.zig");
const Input = inputMod.Input;

const InternalSlc2Meta = extern struct {
    seed: u64,
    padding: [56]u8 = [_]u8{ 0 } ** 56,

    comptime {
        assert(@sizeOf(InternalSlc2Meta) == 64);
    }
};

const Slc2InputType = enum(u3) {
    skip = 0,
    jump,
    left,
    right,
    restart,
    restart_full,
    death,
    tps,

    fn fromCustomAction(action: *inputMod.CustomAction) Slc2InputType {
        // unsupported in slc2
        assert(@as(inputMod.CustomActionTag, action.*) != inputMod.CustomActionTag.bug_checkpoint);

        switch (action) {
            .skip => {
                return .skip;
            },
            .restart => |_| {
            }
        }
    }
    
    pub fn fromAction(action: *inputMod.Action) Slc2InputType {
        switch (action) {
            .custom => |a| {
                return Slc2InputType.fromCustomAction(a);
            }
        }
    }
};

const Slc2Input = union {
    state: u64,
    data: packed struct(u64) {
        frame: u59,
        player2: bool,
        button: Slc2InputType,
        holding: bool
    },

    fn fromRawInput(input: *Input) Slc2Input {
        assert(@as(inputMod.IndexTag, input.*) == inputMod.IndexTag.frame);
        
    }
};

pub const Slc2Meta = struct {
    tps: f64,
    seed: u64
};

const Header = [_]u8{ 0x53, 0x49, 0x4C, 0x4C };

fn calculateBytes(input: *Input) u8 {
    _ = input.index.frame;
}

pub const Slc2Replay = struct {
    inner: Replay(Slc2Meta),
    
    pub const features: Feature = .{
        .tps_change = true,
        .restart = true,
        .restart_full = true,
        .death = true,
        .skip_input = true,
        .set_seed = true,
    };

    pub fn write(self: *const Slc2Replay, _: std.mem.Allocator, writer: anytype) !void {
        try writer.writeAll(&Header);

        // TPS
        try writer.writeInt(u64, @bitCast(self.inner.meta.tps), .little);
        
        // Meta
        //
        // @sizeOf returns a value which for some reason doesn't serialize correctly here
        try writer.writeInt(u64, 64, .little);
        try writer.writeStructEndian(InternalSlc2Meta {
            .seed = self.inner.meta.seed
        }, .little);

        // Length
        try writer.writeInt(u64, @intCast(self.inner.inputs.items.len), .little);
        
        //var blobs = std.ArrayList()
        //
        //
        
        //for (self.inner.inputs) |input| {
        //  
        //}
    }
};
