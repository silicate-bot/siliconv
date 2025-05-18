pub const CustomActionTag = enum {
    skip,
    restart,
    bug_checkpoint,
    change_tps
};

pub const CustomAction = union(CustomActionTag) {
    skip: void,
    restart: struct {
        restart_type: enum {
            restart, restart_full, death
        },
        new_seed: ?u64
    },
    bug_checkpoint: void,
    change_tps: struct {
        tps: f64
    },
};

pub const ActionTag = enum {
    vanilla,
    custom
};

pub const Action = union(ActionTag) {
    vanilla: packed struct(u8) {
        button: u6,
        player2: bool,
        down: bool
    },
    custom: CustomAction
};

pub const PositionData = struct {
    x: f32,
    y: f32,
    rot: f32,

    // In theory, having these set to zero shouldn't
    // break frame-fix macros, as long as the position is set
    // every frame.
    //
    // These are nullable due to Replay Engine not saving velocity.
    x_vel: ?f32,
    y_vel: ?f32
};

pub const IndexTag = enum {
    frame,

    // backwards compatibilty. i'm not too happy about it either
    xpos
};

pub const Index = union(IndexTag) {
    frame: u64,
    xpos: f64,
};

pub const Input = struct {
    index: Index,
    action: Action,
    position: ?PositionData
};
