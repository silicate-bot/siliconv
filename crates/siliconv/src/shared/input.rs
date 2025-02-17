#[derive(Clone)]
pub struct VanillaAction {
    pub button: i32,
    pub player2: bool,
    pub down: bool,
}

#[derive(Clone)]
pub enum RestartType {
    Death,
    Restart,
    RestartFull,
}

#[derive(Clone)]
pub enum CustomAction {
    Skip,
    Restart {
        restart_type: RestartType,
        new_seed: Option<u64>,
    },
    BugCheckpoint,
    ChangeTPS {
        tps: f64,
    },
}

#[derive(Clone)]
pub enum Action {
    Vanilla(VanillaAction),
    Custom(CustomAction),
}

#[derive(Clone)]
pub struct PositionData {
    pub x: f32,
    pub y: f32,
    pub rot: f32,

    // In theory, having these set to zero shouldn't break
    // any macros as long as the position/rotation is set every single frame.
    //
    // As RE does not save x velocity, converting frame fix macros could be an issue.
    pub x_vel: Option<f64>,
    pub y_vel: Option<f64>,
}

/// A generic input.
///
/// Every format-specific input type must be convertible to
/// this type via the [`TryFrom`] trait.
#[derive(Clone)]
pub struct Input {
    pub frame: u64,
    pub action: Action,
    pub position: Option<PositionData>,
}
