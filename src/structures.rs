use std::{ffi::c_int, mem};

use crate::constants::{self, AddressType};

/// Address in MSF format
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Msf {
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
}

impl Msf {
    pub fn to_lba(&self) -> i32 {
        (((self.minute as i32 * constants::CD_SECS) + self.second as i32) * constants::CD_FRAMES + self.frame as i32) - constants::CD_MSF_OFFSET
    }

    pub fn from_lba(lba: i32) -> Self {
        let offset_a = lba + constants::CD_MSF_OFFSET;
        Msf {
            minute: ((offset_a / constants::CD_FRAMES) / 60) as u8,
            second: ((offset_a / constants::CD_FRAMES) % 60) as u8,
            frame: (offset_a % 75) as u8,
        }
    }

    pub fn invalid(&self) -> bool {
        if self.minute == 0 && self.second < 2 {
            true
        } else {
            false
        }
    }
}

/// Address in either MSF or logical format
#[repr(C)]
#[derive(Clone, Copy)]
pub union AddrUnion {
    pub lba: c_int,
    pub msf: Msf,
}

#[derive(Debug, Clone, Copy)]
pub enum Addr {
    Lba(i32),
    Msf(Msf),
}

impl Addr {
    pub fn into_msf(self) -> Msf {
        match self {
            Addr::Lba(a) => Msf::from_lba(a),
            Addr::Msf(msf) => msf,
        }
    }

    pub fn into_lba(self) -> i32 {
        match self {
            Addr::Lba(a) => a,
            Addr::Msf(msf) => msf.to_lba(),
        }
    }
}

/// This struct is used by [`crate::constants::PLAY_MSF`]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MsfLong {
    /// Start minute
    pub min0: u8,
    /// Start second
    pub sec0: u8,
    /// Start frame
    pub frame0: u8,
    /// End minute
    pub min1: u8,
    /// End second
    pub sec1: u8,
    /// End frame
    pub frame1: u8,
}

#[repr(C)]
pub union RawResult {
    pub cdrom_msf: MsfLong,
    pub buffer: *mut u8,
}

/// This struct is used by [`crate::constants::PLAY_TRACK_INDEX`]
#[repr(C)]
struct TrackIndex {
    /// Start track
    trk0: u8,
    /// Start index
    ind0: u8,
    /// End track
    trk1: u8,
    /// End index
    ind1: u8,
}

/// This struct is used by [`crate::constants::READ_TOC_HEADER`]
#[repr(C)]
#[derive(Default, Debug)]
pub struct TocHeader {
    pub first_track: u8,
    pub last_track: u8,
}

// This struct is used by the [`crate::constants::READTOCENTRY`] ioctl
#[repr(C)]
pub(crate) struct _TocEntry {
    pub track: u8,
    pub adr_ctrl: u8,
    pub format: u8,
    pub addr: AddrUnion,
    pub datamode: u8,
}

impl Default for _TocEntry {
    fn default() -> Self {
        unsafe {
            Self {
                track: 0,
                adr_ctrl: 0,
                format: AddressType::Msf as u8,
                addr: mem::zeroed(),
                datamode: 0
            }
        }
    }
}

// Actually public version of [`_TocEntry`].
#[derive(Debug, Clone, Copy)]
pub struct TocEntry {
    pub track: u8,
    pub adr: u8,
    pub ctrl: u8,
    pub addr: Addr,
}

struct VolCtl {

}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReadAudio {
    /// Frame address
    pub addr: AddrUnion,
    /// CDROM_LBA or CDROM_MSF
    pub addr_format: AddressType,
    /// Number of 2352-byte-frames to read at once
    pub nframes: i32,
    /// Pointer to frame buffer (size: nframes*2352 bytes)
    pub buf: *mut i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct _SubChannel {
    pub format: u8,
    pub audiostatus: u8,
    pub adr_ctrl: u8,
    pub trk: u8,
    pub ind: u8,
    pub absaddr: AddrUnion,
    pub reladdr: AddrUnion,
}

impl Default for _SubChannel {
    fn default() -> Self {
        unsafe {
            Self {
                format: AddressType::Msf as u8,
                audiostatus: 0,
                adr_ctrl: 0,
                trk: 0,
                ind: 0,
                absaddr: mem::zeroed(),
                reladdr: mem::zeroed(),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SubChannel {
    pub audiostatus: u8,
    pub adr: u8,
    pub ctrl: u8,
    pub trk: u8,
    pub ind: u8,
    pub absaddr: Addr,
    pub reladdr: Addr,
}
