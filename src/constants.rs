//! Read more about the meanings of the below bytes here:
//! <https://docs.kernel.org/userspace-api/ioctl/cdrom.html>

use num_traits::ToPrimitive;

pub const EDRIVE_CANT_DO_THIS: i32 = nix::errno::Errno::EOPNOTSUPP as i32;

/// CDROM ioctl byte, from <linux/cdrom.h>
pub const IOC_BYTE: u8 = 0x53;

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    /// Pause Audio Operation
    Pause = 0x01,
    /// Resume paused Audio Operation
    Resume = 0x02,
    /// Play Audio MSF (struct cdrom_msf)
    PlayMsf = 0x03,
    /// Play Audio Track/index (struct cdrom_ti)
    PlayTrackIndex = 0x04,
    /// Read TOC header (struct cdrom_tochdr)
    ReadTocHeader = 0x05,
    /// Read TOC entry (struct cdrom_tocentry)
    ReadTocEntry = 0x06,
    /// Stop the cdrom drive
    Stop = 0x07,
    /// Start the cdrom drive
    Start = 0x08,
    /// Ejects the cdrom media
    Eject = 0x09,
    /// Control output volume (struct cdrom_volctrl)
    VolumeControl = 0x0a,
    /// Read subchannel data (struct cdrom_subchnl)
    SubChannel = 0x0b,
    /// Read CDROM mode 2 data (2336 Bytes) (struct cdrom_subchnl)
    ReadMode2 = 0x0c,
    /// Read CDROM mode 1 data (2048 Bytes) (struct cdrom_read)
    ReadMode1 = 0x0d,
    /// (struct cdrom_read_audio)
    ReadAudio = 0x0e,
    /// Enable (1)/Disable (0) auto-ejecting
    EjectSoftware = 0x0f,
    /// Obtain the start-of-last-session address of multi session disks (struct cdrom_multisession)
    MultiSession = 0x10,
    /// Obtain the "Universal Product Code" if available (struct cdrom_mcn)
    GetMcn = 0x11,
    /// Hard-reset the drive
    Reset = 0x12,
    /// Get the drive's volume setting (struct cdrom_volctrl)
    VolumeRead = 0x13,
    /// Read data in raw mode (2352 Bytes) (struct cdrom_read)
    ReadRaw = 0x14,


    // These ioctls are only used in aztcd.c and optcd.c

    /// Read data in cooked mode (???)
    ReadCooked = 0x15,
    /// Seek msf address
    Seek = 0x16,


    // This ioctl is only used by the scsi-cd driver.
    // It is for playing audio in logtal block addresing mode.

    /// (struct cdrom_blk)
    PlayBlock = 0x17,


    // These ioctls are only used in optcd.c

    /// Read all 2646 bytes
    ReadAll = 0x18,


    // These ioctls were only in (now removed) ide-cd.c for controlling
    // drive spindown time.  They should be implemented in the
    // Uniform driver, via generic packet commands, GPCMD_MODE_SELECT_10,
    // GPCMD_MODE_SENSE_10 and the GPMODE_POWER_PAGE...
    // -Erik
    GetSpindown = 0x1d,
    SetSpindown = 0x1e,

    // These ioctls are implemented through the uniform CD-ROM driver
    // They _will_ be adopted by all CD-ROM drivers, when all the CD-ROM
    // drivers are eventually ported to the uniform CD-ROM driver interface.

    /// Pendant of [`Operations::Eject`]
    CloseTray = 0x19,
    /// Set behavior options
    SetOptions = 0x20,
    /// Clear behavior options
    ClearOptions = 0x21,
    /// Set the CD-ROM speed
    SelectSpeed = 0x22,
    /// Select disc (for juke-boxes)
    SelectDisk = 0x23,
    /// Check is media changed
    MediaChanged = 0x25,
    /// Get tray position, etc
    DriveStatus = 0x26,
    /// Get disc type, etc
    DiscStatus = 0x27,
    /// Get number of slots
    ChangerNslots = 0x28,
    /// Lock or unlock door
    LockDoor = 0x29,
    /// Turn debug messages on/off
    Debug = 0x30,
    /// Get capabilities
    GetCapability = 0x31,

    // Note that scsi/scsi_ioctl.h also uses 0x5382 - 0x5386.
    // Future CDROM ioctls should be kept below 0x537F

    /// Set the audio buffer size
    /// conflict with SCSI_IOCTL_GET_IDLUN
    AudioBufferSize = 0x82,

    // DVD-ROM Specific ioctls

    /// Read structure
    DvdReadStructure = 0x90,
    /// Write structure
    DvdWriteStructure = 0x91,
    /// Authentication
    DvdAuthenticate = 0x92,

    /// Send a packet to the drive
    SendPacket = 0x93,
    /// Get next writable block
    NextWritable = 0x94,
    /// Get last block written on disc
    LastWritten = 0x95,
    /// Get the timestamp of the last media change
    TimedMediaChange = 0x96,
}

/// Turns the byte into its full IOCTL representation
pub fn op_to_ioctl(op: Operation) -> u64 {
    let value = op.to_u8().unwrap();

    ((IOC_BYTE as u64) << 8) + value as u64
}

/// Drive status possibilities returned by CDROM_DRIVE_STATUS ioctl
#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    NoInfo = 0,
    NoDisc = 1,
    TrayOpen = 2,
    DriveNotReady = 3,
    DiscOK = 4
}

/// Disc status possibilities returned by CDROM_DISC_STATUS ioctl
#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiscType {
    NoInfo = 0,
    Audio = 100,
    Data1 = 101,
    Data2 = 102,
    XA21  = 103,
    XA22  = 104,
    Mixed = 105,
}

pub const CD_MINS: i32 = 74;
pub const CD_SECS: i32 = 60;
pub const CD_FRAMES: i32 = 75;
pub const CD_SYNC_SIZE: i32 = 12;
pub const CD_MSF_OFFSET: i32 = 150;
pub const CD_CHUNK_SIZE: i32 = 24;
pub const CD_NUM_OF_CHUNKS: i32 = 98;
pub const CD_FRAMESIZE_SUB: i32 = 96;
pub const CD_HEAD_SIZE: i32 = 4;
pub const CD_SUBHEAD_SIZE: i32 = 8;
pub const CD_EDC_SIZE: i32 = 4;
pub const CD_ZERO_SIZE: i32 = 8;
pub const CD_ECC_SIZE: i32 = 276;
pub const CD_FRAMESIZE: i32 = 2048;
pub const CD_FRAMESIZE_RAW: i32 = 2352;
pub const CD_FRAMESIZE_RAWER: i32 = 2646;
pub const CD_FRAMESIZE_RAW1: i32 = CD_FRAMESIZE_RAW - CD_SYNC_SIZE;
pub const CD_FRAMESIZE_RAW0: i32 = CD_FRAMESIZE_RAW - CD_SYNC_SIZE - CD_HEAD_SIZE;

pub const CD_XA_HEAD: i32 = CD_HEAD_SIZE + CD_SUBHEAD_SIZE;
pub const CD_XA_TAIL: i32 = CD_EDC_SIZE + CD_ECC_SIZE;
pub const CD_XA_SYNC_HEAD: i32 = CD_SYNC_SIZE + CD_XA_HEAD;

#[repr(u8)]
#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressType {
    Lba = 0x01,
    Msf = 0x02,
}

#[derive(FromPrimitive, ToPrimitive)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioStates {
    Invalid = 0x00,
    Play = 0x11,
    Paused = 0x12,
    Completed = 0x13,
    Error = 0x14,
    NoStatus = 0x15,
}

pub enum Capability {
    CloseTray = 0x01,
    OpenTray = 0x02,
    Lock = 0x04,
    SelectSpeed = 0x08,
    SelectDisc = 0x10,
    MultiSession = 0x20,
    Mcn = 0x40,
    MediaChanged = 0x80,
    PlayAudio = 0x100,
    Reset = 0x200,
    DriveStatus = 0x800,
    GenericPacket = 0x1000,
    CdR = 0x2000,
    CdRW = 0x4000,
    Dvd = 0x8000,
    DvdR = 0x10000,
    DvdRam = 0x20000,
    MODrive = 0x40000,
    Mrw = 0x80000,
    MrwW = 0x1000000,
    Ram = 0x2000000,
}

pub enum GenericPacketCommand {

}
