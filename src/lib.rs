use constants::AddressType;
use thiserror::Error;

use crate::{constants::{DiscType, Status}, structures::{Addr, SubChannel, TocEntry, TocHeader}};

#[macro_use]
extern crate num_derive;

pub mod constants;
pub mod structures;
pub mod platform;

#[cfg(target_os="linux")]
pub type CDRom = platform::linux::CDRomLinux;

#[cfg(target_os="windows")]
pub type CDRom = platform::windows::CDRomWindows;

#[derive(Error, Debug, Clone)]
pub enum CDRomError {
    #[cfg(target_os="linux")]
    #[error("internal system error")]
    Errno(#[from] nix::errno::Errno),

    #[error("no disc in drive to read")]
    NoDisc,

    #[error("the CD does not contain cd-audio")]
    NotAudioCD,

    #[error("the drive's door is locked for some reason")]
    DoorLocked,

    #[error("this drive does not support the function")]
    Unsupported,

    #[error("the drive is in use by another user")]
    Busy,

    #[error("the address specified was invalid")]
    InvalidAddress,

    #[error("the buffer size was too small; needed at least {0} bytes, got {1} bytes")]
    InvalidBufferSize(usize, usize),
}


pub trait CDRomTrait {
    /// Get the currently reported status of the drive.
    fn status(&mut self) -> Option<Status>;

    /// Get the type of disc currently in the drive
    fn disc_type(&mut self) -> Option<DiscType>;

    /// Get the Media Catalog Number of the current disc.
    ///
    /// Many discs do not contain this information.
    fn mcn(&mut self) -> Option<String>;

    fn toc_header(&mut self) -> Result<TocHeader, CDRomError>;

    fn toc_entry(&mut self, index: u8, address_type: AddressType) -> TocEntry;

    fn set_lock(&mut self, locked: bool) -> Result<(), CDRomError>;

    fn eject(&mut self) -> Result<(), CDRomError>;

    fn close(&mut self) -> Result<(), CDRomError>;

    fn subchannel(&mut self) -> Result<SubChannel, CDRomError>;

    /// Read audio from the CD.
    ///
    /// This method is a convenience method around [`CDRom::read_audio_into`].
    fn read_audio(&mut self, address: Addr, frames: usize) -> Result<Vec<i16>, CDRomError>;

    /// Read audio from the CD into a preallocated buffer.
    ///
    /// The buffer must be large enough to hold the audio for all the frames you want to read.
    /// Since the values are [`i16`]s, the equation for the buffer size is `(n_frames * 2352) / 2`
    fn read_audio_into(
        &mut self,
        address: Addr,
        frames: usize,
        buf: &mut [i16]
    ) -> Result<(), CDRomError>;

    fn read_raw_into(
        &mut self,
        address: Addr,
        buf: &mut [u8]
    ) -> Result<(), CDRomError>;
}