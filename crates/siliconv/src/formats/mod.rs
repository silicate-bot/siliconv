use std::io::{Read, Seek, Write};

use crate::shared::{error::Error, meta::ReplayMeta, replay::Replay};

pub mod silicate;

pub trait ReplaySerializable<M: ReplayMeta> {
    fn from_reader<R>(reader: R) -> Result<Self, Error>
    where
        Self: Sized,
        R: Read + Seek;

    fn to_writer<W>(&mut self, writer: W) -> Result<(), Error>
    where
        Self: Sized,
        W: Write + Seek;

    fn inner(&self) -> &Replay<M>;
    fn inner_mut(&mut self) -> &mut Replay<M>;
}
