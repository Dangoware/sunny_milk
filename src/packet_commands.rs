use std::{fmt::{write, Debug}, os::raw::c_void};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GenericCommand {
    pub cdb: [u8; CDROM_PACKET_SIZE],
    pub buffer: *mut u8,
    pub buflen: u32,
    pub stat: i32,
    pub sense: *mut RequestSense,
    pub data_direction: DataDirection,
    pub quiet: i32,
    pub timeout: i32,
    pub u: U
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RequestSense {
    pub valid_error: ValidError,
    pub segment_number: u8,
    pub reserved: Reserved,
    pub information: [u8; 4],
    pub addditional_sense_len: u8,
    pub command_info: [u8; 4],
    pub asc: u8,
    pub ascq: u8,
    pub fruc: u8,
    pub sks: [u8; 3],
    pub asb: [u8; 46]
}

#[cfg(target_endian = "big")]
#[bitfield_struct::bitfield(u8)]
pub struct ValidError {
    #[bits(1)]
    pub valid: u8,
    #[bits(7)]
    pub error_code: u8,
}

#[cfg(target_endian = "little")]
#[bitfield_struct::bitfield(u8)]
pub struct ValidError {
    #[bits(7)]
    pub error_code: u8,
    #[bits(1)]
    pub valid: u8,
}

#[cfg(target_endian = "big")]
#[bitfield_struct::bitfield(u8)]
pub struct Reserved {
    #[bits(2)]
    pub reserved1: u8,
    #[bits(1)]
    pub ili: u8,
    #[bits(1)]
    pub reserved2: u8,
    #[bits(4)]
    pub sense_key: u8
}

#[cfg(target_endian = "little")]
#[bitfield_struct::bitfield(u8)]
pub struct Reserved {
    #[bits(4)]
    pub sense_key: u8,
    #[bits(1)]
    pub reserved2: u8,
    #[bits(1)]
    pub ili: u8,
    #[bits(2)]
    pub reserved1: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union U {
    pub reserved: *const [c_void; 1],
    pub unused: *const c_void
}

impl Debug for U {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write(f, format_args!("type U here"))
    }
}

pub const CDROM_PACKET_SIZE: usize = 12;
/// The generic packet command opcodes for CD/DVD Logical Units,
/// From Table 57 of the SFF8090 Ver. 3 (Mt. Fuji) draft standard.
#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericPacketCommand {
    Blank = 0xa1,
    CloseTrack = 0x5b,
    FlushCache = 0x35,
    FormatUnit = 0x04,
    GetConfiguration = 0x46,
    GetEventStatus = 0x4a,
    GetPerformance = 0xac,
    GetInquiry = 0x12,
    LoadUnload = 0xa6,
    MechanismStatus = 0xbd,
    ModeSelect10 = 0x55,
    ModeSense10 = 0x5a,
    PauseResume = 0x4b,
    PlayAudio10 = 0x45,
    PlayAudioMsf = 0x47,

    /// This seems to be a SCSI specific CD-ROM opcode
    /// to play data at track/index
    PlayAudioTi = 0x48,
    PlayCd = 0xbc,
    PreventAllowMediumRemoval = 0x1e,
    Read10 = 0x28,
    Read12 = 0xa8,
    ReadBuffer = 0x3c,
    ReadBufferCapacity = 0x5c,
    ReadCdvdCapacity = 0x25,
    ReadCd = 0xbe,
    ReadCdMsf = 0xb9,
    ReadDiscInfo = 0x51,
    ReadDvdStructure = 0xad,
    ReadFormatCapabilities = 0x23,
    ReadHeader = 0x44,
    ReadTrackRzoneInfo = 0x52,
    ReadSubchannel = 0x42,
    ReadTocPmaAtip = 0x43,
    RepairRzoneTrack = 0x58,
    ReportKey = 0xa4,
    RequestSense = 0x03,
    ReserveRzoneTrack = 0x53,
    SendCueSheet = 0x5d,
    Scan = 0xba,
    Seek = 0x2b,
    SendDvdStructure = 0xbf,
    SendEvent = 0xa2,
    SendKey = 0xa3,
    SendOpc = 0x54,
    SetReadAhead = 0xa7,
    SetStreaming = 0xb6,
    StartStopUnit = 0x1b,
    StopPlayScan = 0x4e,
    TestUnitReady = 0x00,
    Verify10 = 0x2f,
    Write10 = 0x2a,
    Write12 = 0xaa,
    WriteAndVerify10 = 0x2e,
    WriteBuffer = 0x3b,

    /// This is listed as optional in ATAPI 2.6, but is (curiously)
    /// missing from Mt. Fuji, Table 57.  It _is_ mentioned in Mt. Fuji
    /// Table 377 as an MMC command for SCSi devices though...  Most ATAPI
    /// drives support it.
    SetSpeed = 0xbb,

    /// From MS Media Status Notification Support Specification. For
    /// older drives only.
    GetMediaStatus = 0xda,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum DataDirection {
    Unknown,
    Write,
    Read,
    None
}