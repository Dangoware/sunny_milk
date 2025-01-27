use std::ffi::c_int;

/// Address in MSF format
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Msf0 {
    pub minute: u8,
    pub second: u8,
    pub frame: u8,
}

/// Address in either MSF or logical format
#[repr(C)]
#[derive(Clone, Copy)]
pub union Addr {
    pub lba: c_int,
    pub msf: Msf0,
}

/// This struct is used by [`crate::constants::PLAY_MSF`]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Msf {
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
    pub cdrom_msf: Msf,
    pub buffer: [u8; 2646],
}

/// This struct is used by [`crate::constants::PLAY_TRACK_INDEX`]
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
struct TocHeader {
    trk0: u8,
    trk1: u8,
}

struct VolCtl {

}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ReadAudio {
    /// Frame address
    pub addr: Addr,
    /// CDROM_LBA or CDROM_MSF
    pub addr_format: u8,
    /// Number of 2352-byte-frames to read at once
    pub nframes: i32,
    /// Pointer to frame buffer (size: nframes*2352 bytes)
    pub buf: *mut u8,
}
