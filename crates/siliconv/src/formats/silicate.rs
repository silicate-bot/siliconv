use std::io::{Read, Seek};

use crate::shared::{
    error::Error,
    feature::ReplayFeature,
    format::ReplayFormat,
    input::{Action, CustomAction, Input, RestartType, VanillaAction},
    meta::ReplayMeta,
    replay::Replay,
    version::Version,
};

use super::ReplaySerializable;

struct Slc2Blob {
    pub byte_size: u64,
    pub start: u64,
    pub len: u64,
}

impl Slc2Blob {
    pub fn from_reader<R>(reader: &mut R) -> Result<Self, Error>
    where
        R: Read + Seek,
    {
        let mut long_buf = [0u8; 8];

        reader.read_exact(&mut long_buf)?;
        let byte_size = u64::from_le_bytes(long_buf);

        reader.read_exact(&mut long_buf)?;
        let start = u64::from_le_bytes(long_buf);

        reader.read_exact(&mut long_buf)?;
        let len = u64::from_le_bytes(long_buf);

        Ok(Self {
            byte_size,
            start,
            len,
        })
    }

    pub fn read_inputs_to_vec<R>(
        &self,
        reader: &mut R,
        frame: &mut u64,
        inputs: &mut Vec<Input>,
    ) -> Result<(), Error>
    where
        R: Read + Seek,
    {
        let mut state_buf = [0u8; 8];

        for _ in self.start..(self.start + self.len) {
            reader.read_exact(&mut state_buf)?;
            let state = u64::from_le_bytes(state_buf);

            *frame += state >> 5;

            let action = match (state & 0b11100) >> 2 {
                btn @ 1..=3 => Action::Vanilla(VanillaAction {
                    button: btn as i32,
                    player2: ((state & 2) >> 1) != 0,
                    down: (state & 1) != 0,
                }),
                4 => Action::Custom(CustomAction::Restart {
                    restart_type: RestartType::Restart,
                    new_seed: None,
                }),
                5 => Action::Custom(CustomAction::Restart {
                    restart_type: RestartType::RestartFull,
                    new_seed: None,
                }),
                6 => Action::Custom(CustomAction::Restart {
                    restart_type: RestartType::Death,
                    new_seed: None,
                }),
                7 => {
                    reader.read_exact(&mut state_buf)?;
                    let tps = f64::from_le_bytes(state_buf);

                    Action::Custom(CustomAction::ChangeTPS { tps })
                }
                _ => Action::Custom(CustomAction::Skip),
            };

            inputs.push(Input {
                frame: *frame,
                action,
                position: None,
            });
        }

        Ok(())
    }
}

pub struct SilicateMeta {
    pub tps: f64,
    pub seed: u64,
}

impl ReplayMeta for SilicateMeta {}

// slc (the macro format base) is NOT the current Silicate format
// Therefore we may assume its meta structure, which we may not do with
// regular slc.
pub struct SilicateReplay {
    inner: Replay<SilicateMeta>,
}

const SLC2_FEATURES: ReplayFeature = ReplayFeature::new(
    ReplayFeature::DEATH
        | ReplayFeature::RESTART
        | ReplayFeature::RESTART_FULL
        | ReplayFeature::FPS_CHANGE
        | ReplayFeature::SKIP_INPUT,
);

impl SilicateReplay {
    fn parse_slc1<R>(reader: R) -> Result<Self, crate::shared::error::Error>
    where
        Self: Sized,
        R: std::io::Read + std::io::Seek,
    {
        todo!()
    }

    fn parse_slc2<R>(mut reader: R) -> Result<Self, crate::shared::error::Error>
    where
        Self: Sized,
        R: std::io::Read + std::io::Seek,
    {
        // Header actually has size 5 because of null terminator
        reader.seek(std::io::SeekFrom::Current(5))?;

        let mut long_buf = [0u8; 8];

        // TPS (f64)
        reader.read_exact(&mut long_buf)?;
        let tps = f64::from_le_bytes(long_buf);

        // Meta size (u64)
        reader.read_exact(&mut long_buf)?;
        let meta_size = u64::from_le_bytes(long_buf);

        // Seed (u64) - in meta originally
        reader.read_exact(&mut long_buf)?;
        let seed = u64::from_le_bytes(long_buf);

        // Skip the rest of meta (reserved atm)
        reader.seek(std::io::SeekFrom::Current(56))?;

        // Replay length (u64)
        reader.read_exact(&mut long_buf)?;
        let length = u64::from_le_bytes(long_buf);

        let mut blobs: Vec<Slc2Blob> = vec![];

        // Replay length (u64)
        reader.read_exact(&mut long_buf)?;
        let blob_count = u64::from_le_bytes(long_buf);

        for _ in 0..blob_count {
            blobs.push(Slc2Blob::from_reader(&mut reader)?);
        }

        let mut inputs: Vec<Input> = vec![];
        inputs.reserve(length as usize);

        let mut current_frame = 0u64;

        blobs.iter().try_for_each(|blob| {
            blob.read_inputs_to_vec(&mut reader, &mut current_frame, &mut inputs)
        })?;

        Ok(SilicateReplay {
            inner: Replay {
                features: SLC2_FEATURES,
                original_format: ReplayFormat::Silicate,
                version: Version {
                    major: 22,
                    minor: 074,
                },
                meta: Box::new(SilicateMeta { tps, seed }),
                inputs,
            },
        })
    }
}

const SLC2_HEADER: [u8; 4] = [83, 73, 76, 76];

impl ReplaySerializable<SilicateMeta> for SilicateReplay {
    fn inner(&self) -> &Replay<SilicateMeta> {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Replay<SilicateMeta> {
        &mut self.inner
    }

    fn to_writer<W>(&mut self, writer: W) -> Result<(), crate::shared::error::Error>
    where
        Self: Sized,
        W: std::io::Write + std::io::Seek,
    {
        todo!()
    }

    fn from_reader<R>(mut reader: R) -> Result<Self, crate::shared::error::Error>
    where
        Self: Sized,
        R: std::io::Read + std::io::Seek,
    {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        reader.seek(std::io::SeekFrom::Start(0))?;

        if header == SLC2_HEADER {
            Self::parse_slc2(reader)
        } else {
            Self::parse_slc1(reader)
        }
    }
}
