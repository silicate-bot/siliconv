use std::num::NonZero;

use super::{
    error::Error,
    feature::ReplayFeature,
    format::ReplayFormat,
    input::{Action, CustomAction, Input},
    meta::ReplayMeta,
    version::Version,
};

#[derive(Clone)]
pub struct Replay<M: ReplayMeta> {
    pub features: ReplayFeature,
    pub original_format: ReplayFormat,
    pub version: Version,

    pub meta: Box<M>,
    pub inputs: Vec<Input>,
}

impl<'a, M: ReplayMeta> Replay<M> {
    pub fn load<R>(replay: R) -> Result<Self, Error>
    where
        R: TryInto<Replay<M>, Error = Error>,
    {
        replay.try_into()
    }

    pub fn dump<R>(self) -> Result<R, Error>
    where
        R: TryFrom<Replay<M>, Error = Error>,
    {
        R::try_from(self)
    }

    fn is_death_input(input: &Input) -> bool {
        if let Action::Custom(ref action) = input.action {
            return matches!(action, CustomAction::Restart { .. });
        } else {
            false
        }
    }

    fn determine_attempt_pos(&self, attempt: NonZero<u32>) -> Option<usize> {
        let deaths = attempt.get() - 1;
        let mut position: usize = 0;

        {
            let mut it = self.inputs.iter();

            for _ in 0..deaths {
                position = it.position(Self::is_death_input)?;
            }
        }

        Some(position)
    }

    pub fn iter_for_attempt(
        &'a self,
        attempt: NonZero<u32>,
    ) -> Option<Box<impl Iterator<Item = &'a Input>>> {
        let position = self.determine_attempt_pos(attempt)?;

        Some(Box::new(
            self.inputs
                .iter()
                .skip(position + 1)
                .take_while(|input| !Self::is_death_input(input)),
        ))
    }

    pub fn iter_mut_for_attempt(
        &'a mut self,
        attempt: NonZero<u32>,
    ) -> Option<Box<impl Iterator<Item = &'a mut Input>>> {
        let position = self.determine_attempt_pos(attempt)?;

        Some(Box::new(
            self.inputs
                .iter_mut()
                .skip(position + 1)
                .take_while(|input| !Self::is_death_input(input)),
        ))
    }
}
