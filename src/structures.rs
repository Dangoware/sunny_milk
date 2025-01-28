use std::{ffi::c_int, mem};

use crate::constants::AddressType;

/// Address in MSF format
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Msf {
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
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
    pub buffer: [u8; 2352],
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
#[derive(Debug)]
pub struct TocEntry {
    pub track: u8,
    pub adr: u8,
    pub ctrl: u8,
    pub addr: Addr,
    pub datamode: u8,
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

/// This struct is used with the CDROM_GET_MCN ioctl.
/// Very few audio discs actually have Universal Product Code information,
/// which should just be the Medium Catalog Number on the box.  Also note
/// that the way the codeis written on CD is _not_ uniform across all discs!
struct Mcn {
    medium_catalog_number: [u8; 14]
}
