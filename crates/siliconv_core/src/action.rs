//! Module containing action-related types for replays.

/// A player button.
pub enum PlayerButton {
    /// A jump. Equivalent to the button 1.
    Jump,
    /// A left input. Equivalent to the button 2.
    Left,
    /// A right input. Equivalent to the button 3.
    Right,
}

/// A restart type.
///
/// One of [`RestartType::Restart`], [`RestartType::RestartFull`] or [`RestartType::Death`].
pub enum RestartType {
    /// The [`RestartType::Restart`] type. Typically respawns at the last platformer checkpoint.
    Restart,
    /// The [`RestartType::RestartFull`] type. Typically respawns at the level beginning.
    RestartFull,
    /// The [`RestartType::Death`] type. Typically marks that the player should die.
    Death,
}

/// An action in the replay.
pub enum Action {
    /// An empty action. Doesn't do anything.
    Empty,
    /// A player action. Corresponds to a button press.
    Player {
        /// The button to press/release.
        button: PlayerButton,
        /// Whether to press/release the button.
        hold: bool,
        /// Whether this action is for player 1 or 2.
        player2: bool,
    },
    /// A restart action. Either forces a restart or signifies player death.
    Restart {
        /// The restart type.
        restart_type: RestartType,
        /// The new (optional) seed to set after the restart.
        seed: Option<u64>,
    },
    /// A TPS action. Changes the TPS of the replay.
    TPS {
        /// The new TPS to set.
        tps: f64,
    },
    /// A bugpoint action. Places down a bugged checkpoint at this current frame.
    Bugpoint,
}

/// A time point at which to execute actions.
pub enum TimePoint {
    /// A frame time point.
    ///
    /// Note that frame counting methods may differ per bot.
    /// The most popular methods for frame counting contain:
    /// - `playLayer->m_currentProgress` (as per the Geode bindings)
    /// - `playLayer->m_levelTime / tps`
    /// - custom frame counters based on update ticks (personally recommend this one the most :D)
    ///
    /// If you're designing a new replay format, opt to use frames.
    Frame(u64),

    /// An x position time point.
    /// This denotes the x position of **Player 1**.
    /// As the two positions can be desynced even in versions 2.1 and prior,
    /// please keep this in mind when designing a converter to the intermediate format.
    ///
    /// Less reliable than frame counting.
    ///
    /// This time point technically works on game versions 2.2 and above,
    /// but may cause action overlap with direction triggers.
    ///
    /// This time point is only supported for compatibility reasons.
    /// If you're designing a new replay format, please don't use this.
    XPos(f64),

    /// A relative timestamp time format.
    ///
    /// This time point is only supported for compatibility.
    /// Don't use this time point. It's inaccurate and highly inefficient.
    Time(f64),
}

/// Player position data for "frame-fixed" replays.
///
/// When designing a new replay format, consider if you really need this functionality.
pub struct PlayerPosition {
    /// The X position of the player.
    pub x: f64,
    /// The Y position of the player.
    pub y: f64,

    /// The rotation of the player.
    pub rotation: Option<f64>,
    /// The X velocity of the player.
    pub vel_x: Option<f64>,
    /// The Y velocity of the player.
    pub vel_y: Option<f64>,
}

/// Position data for both players used for "frame-fixed" replays.
///
/// When designing a new replay format, consider if you really need this functionality.
pub struct Position {
    /// Player 1 position data.
    pub player1: PlayerPosition,
    /// Player 2 position data.
    pub player2: PlayerPosition,
}

/// An action with a given time point.
///
/// May contain optional player position data.
pub struct TimedAction {
    /// The time point at which to perform this action.
    pub time: TimePoint,
    /// What action to take during this time point.
    pub action: Action,
    /// The position at which both players are in this time point.
    pub position: Option<Position>,
}
