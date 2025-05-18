const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const lib_mod = b.createModule(.{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
        .optimize = optimize,
    });

    const lib = b.addLibrary(.{
        .linkage = .static,
        .name = "siliconv",
        .root_module = lib_mod,
    });
    b.installArtifact(lib);

    const lib_check = b.addLibrary(.{
        .linkage = .static,
        .name = "siliconv",
        .root_module = lib_mod,
    });

    const exe_mod = b.createModule(.{
        .root_source_file = b.path("src/main.zig"),
        .target = target,
        .optimize = optimize,
    });

    const exe = b.addExecutable(.{
        .name = "siliconv-cli",
        .root_module = exe_mod
    });
    b.installArtifact(exe);

    const exe_check = b.addExecutable(.{
        .name = "siliconv-cli",
        .root_module = exe_mod
    });

    const check = b.step("check", "Check if siliconv compiles");
    check.dependOn(&lib_check.step);
    check.dependOn(&exe_check.step);
}
