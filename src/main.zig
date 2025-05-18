const std = @import("std");
const slc = @import("formats/silicate.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    const replay = slc.Slc2Replay {
        .inner = .{
            .features = slc.Slc2Replay.features,
            .orig_fmt = .slc2,
            .meta = .{
                .tps = 240.0,
                .seed = 6969,
            },
            .version = .latest,
            .inputs = .init(allocator),
        },
    };

    const fd = try std.fs.cwd().openFile("whale.slc", .{
        .mode = .read_write,
        .lock = .exclusive
    });
    defer fd.close();
    
    try replay.write(fd.writer());
}
