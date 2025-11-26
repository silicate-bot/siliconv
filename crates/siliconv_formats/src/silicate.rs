use std::io::{Read, Seek, Write};

use siliconv_core::{
    action::{Action, PlayerButton, RestartType, TimePoint, TimedAction},
    error::ReplayError,
    format::Format,
    replay::{Replay, ReplaySerializable},
    version::GameVersion,
};
use siliconv_macros::Meta;
use slc_oxide::{self as slc, v3::ActionType};

#[derive(Meta)]
pub struct SilicateMeta {
    #[meta(default = 240.0)]
    pub tps: f64,
    pub seed: u64,
}

pub struct SilicateReplay {
    inner: Replay,
}

impl ReplaySerializable for SilicateReplay {
    fn new(replay: Replay) -> Self {
        SilicateReplay { inner: replay }
    }

    fn into_replay(self) -> Replay {
        self.inner
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, ReplayError>
    where
        Self: Sized,
    {
        use slc::v3 as slc;

        let original = slc::Replay::read(reader)
            .map_err(|e| ReplayError::ReadError(format!("failed to read slc3 replay: {e}")))?;

        let action_atom = original
            .atoms
            .atoms
            .iter()
            .find_map(|atom| {
                if let slc::atom::AtomVariant::Action(action_atom) = atom {
                    Some(action_atom)
                } else {
                    None
                }
            })
            .ok_or(ReplayError::ReadError(
                "missing action atom in slc3 replay".to_string(),
            ))?;

        let meta = SilicateMeta {
            tps: original.metadata.tps,
            seed: original.metadata.seed,
        };

        let actions = action_atom
            .actions
            .iter()
            .map(|a| TimedAction {
                time: TimePoint::Frame(a.frame),
                action: match a.action_type {
                    ActionType::Jump => Action::Player {
                        button: PlayerButton::Jump,
                        hold: a.holding,
                        player2: a.player2,
                    },
                    ActionType::Left => Action::Player {
                        button: PlayerButton::Left,
                        hold: a.holding,
                        player2: a.player2,
                    },
                    ActionType::Right => Action::Player {
                        button: PlayerButton::Right,
                        hold: a.holding,
                        player2: a.player2,
                    },
                    ActionType::Restart => Action::Restart {
                        restart_type: RestartType::Restart,
                        seed: Some(a.seed),
                    },
                    ActionType::RestartFull => Action::Restart {
                        restart_type: RestartType::RestartFull,
                        seed: Some(a.seed),
                    },
                    ActionType::Death => Action::Restart {
                        restart_type: RestartType::Death,
                        seed: Some(a.seed),
                    },
                    ActionType::TPS => Action::TPS { tps: a.tps },
                    ActionType::Reserved => Action::Empty,
                },
                position: None,
            })
            .collect();

        Ok(SilicateReplay {
            inner: Replay {
                meta: Box::new(meta),
                actions,
                format: Format::Slc3,
                game_version: GameVersion::new(22, 74),
            },
        })
    }

    fn write<W: Write>(&self, _writer: &mut W) -> Result<(), ReplayError> {
        todo!()
    }
}
