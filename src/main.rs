mod constants;
mod structures;

use std::io::Write;
use std::os::fd::RawFd;
use std::os::{fd::IntoRawFd, unix::fs::OpenOptionsExt};
use std::fs::OpenOptions;
use std::ptr::addr_of_mut;

use constants::{op_to_ioctl, AddressType, DiscStatus, Operation, Status};
use nix::errno::Errno;
use nix::{ioctl_none_bad, ioctl_read_bad, ioctl_readwrite_bad, ioctl_write_int_bad, libc};

use num_traits::FromPrimitive as _;
use structures::{Addr, AddrUnion, Msf, MsfLong, RawResult, ReadAudio, TocEntry, TocHeader, _TocEntry};
use thiserror::Error;

#[macro_use]
extern crate num_derive;

fn main() {
    let mut cd_rom = CDRom::new().unwrap();

    let status = cd_rom.status().unwrap();
    println!("Drive status: {:?}", status);
    cd_rom.close().unwrap();

    println!("Disc status: {:?}", cd_rom.disc_type());

    let header = cd_rom.toc_header().unwrap();
    for i in header.first_track..header.last_track {
        let entry = cd_rom.toc_entry(i);
        println!("{:?}", entry.addr);
    }

    cd_rom.set_lock(false).unwrap();
}

fn rip_cd() {
    let mut cd_rom = CDRom::new().unwrap();

    println!("Drive status: {:?}", cd_rom.status().unwrap());
    println!("Disc status: {:?}", cd_rom.disc_type().unwrap());

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("cd_audio.wav", spec).unwrap();
    let mut buf = vec![0i16; (75 * constants::CD_FRAMESIZE_RAW as usize) / 2];

    // Checksum calculator
    let mut context = md5::Context::new();

    println!("Begin ripping\n");
    for i in 0..4500u32 {
        let minute = ((i + 2) / 60) as u8;
        let second = ((i + 2) % 60) as u8;
        print!("Reading {minute:02}:{second:02}\r");
        std::io::stdout().flush().unwrap();

        match cd_rom.read_audio_into(
            Addr::Msf(Msf { minute, second, frame: 0 }),
            constants::CD_FRAMES as usize,
            &mut buf,
        ) {
            Ok(s) => s,
            Err(_) => break,
        };

        let audio_u8: Vec<u8> = buf.iter().flat_map(|s| s.to_le_bytes()).collect();
        context.consume(audio_u8);

        let mut writer = writer.get_i16_writer(buf.len() as u32);
        for sample in &buf {
            writer.write_sample(*sample);
        }
        writer.flush().unwrap();
    }

    let sum = context.compute();
    let mut string_sum = String::new();
    sum.iter().for_each(|b| string_sum.push_str(format!("{:0x}", b).as_str()));
    println!("\nFinished!\n\n\tChecksum: {}", string_sum);

    writer.flush().unwrap();
    writer.finalize().unwrap();
}

/// Access to a CD-ROM drive on the system.
pub struct CDRom {
    drive_fd: RawFd,
}

#[derive(Error, Debug, Clone)]
pub enum CDRomError {
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
}

ioctl_none_bad!(cdrom_stop, op_to_ioctl(Operation::Stop));
ioctl_none_bad!(cdrom_start, op_to_ioctl(Operation::Start));
ioctl_none_bad!(cdrom_eject, op_to_ioctl(Operation::Eject));
ioctl_write_int_bad!(cdrom_lock_door, op_to_ioctl(Operation::LockDoor));
ioctl_none_bad!(cdrom_close_tray, op_to_ioctl(Operation::CloseTray));
ioctl_none_bad!(cdrom_status, op_to_ioctl(Operation::DriveStatus));
ioctl_none_bad!(cdrom_disc_status, op_to_ioctl(Operation::DiscStatus));
ioctl_readwrite_bad!(cdrom_read_audio, op_to_ioctl(Operation::ReadAudio), structures::ReadAudio);
ioctl_readwrite_bad!(cdrom_read_raw, op_to_ioctl(Operation::ReadRaw), structures::RawResult);
ioctl_read_bad!(cdrom_get_mcn, op_to_ioctl(Operation::GetMcn), [u8; 14]);
ioctl_read_bad!(cdrom_read_toc_header, op_to_ioctl(Operation::ReadTocHeader), structures::TocHeader);
ioctl_read_bad!(cdrom_read_toc_entry, op_to_ioctl(Operation::ReadTocEntry), structures::_TocEntry);

impl CDRom {
    /// Creates a new interface to a system CD-ROM drive.
    pub fn new() -> Option<Self> {
        let drive_file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK | libc::O_RDONLY)
            .open("/dev/sr0")
            .ok()?;

        Some(Self {
            drive_fd: drive_file.into_raw_fd(),
        })
    }

    /// Get the currently reported status of the drive.
    pub fn status(&mut self) -> Option<Status> {
        let status = unsafe {
            cdrom_status(self.drive_fd).unwrap()
        };

        Status::from_i32(status)
    }

    /// Get the type of disc currently in the drive
    pub fn disc_type(&mut self) -> Option<DiscStatus> {
        let status = unsafe {
            cdrom_disc_status(self.drive_fd).ok()?
        };

        DiscStatus::from_i32(status)
    }

    /// Get the Media Catalog Number of the current disc.
    ///
    /// Many discs do not contain this information.
    pub fn mcn(&mut self) -> Option<String> {
        let mut buffer = [0u8; 14];

        unsafe {
            cdrom_get_mcn(self.drive_fd, addr_of_mut!(buffer)).ok()?;
        }

        let string = String::from_utf8_lossy(&buffer[..buffer.len() - 1]).into_owned();
        Some(string)
    }

    pub fn toc_header(&mut self) -> Result<TocHeader, CDRomError> {
        let mut header = TocHeader::default();

        if unsafe {
            cdrom_read_toc_header(self.drive_fd, addr_of_mut!(header))
        }.is_err_and(|e| e == Errno::ENOMEDIUM) {
            return Err(CDRomError::NoDisc)
        }

        Ok(header)
    }

    pub fn toc_entry(&mut self, index: u8) -> TocEntry {
        let mut header = _TocEntry::default();
        header.track = index;
        header.format = AddressType::Lba as u8;

        unsafe {
            cdrom_read_toc_entry(self.drive_fd, addr_of_mut!(header)).unwrap();
        }

        let entry = TocEntry {
            track: header.track,
            adr: header.adr_ctrl >> 4,
            ctrl: header.adr_ctrl & 0x0F,
            addr: unsafe {
                match header.format {
                    d if d == AddressType::Lba as u8 => Addr::Lba(header.addr.lba),
                    d if d == AddressType::Msf as u8 => Addr::Msf(header.addr.msf),
                    _ => panic!("Impossible value returned!")
                }
            },
            datamode: header.datamode,
        };

        entry
    }

    pub fn set_lock(&mut self, locked: bool) -> Result<(), CDRomError> {
        let result = match unsafe {
            cdrom_lock_door(self.drive_fd, locked as i32)
        } {
            Ok(v) => v,
            Err(e) => match e {
                Errno::EBUSY => return Err(CDRomError::Busy),
                _ => return Err(CDRomError::Errno(e)),
            },
        };

        match result {
            constants::EDRIVE_CANT_DO_THIS => Err(CDRomError::Unsupported),
            _ => Ok(())
        }
    }

    pub fn eject(&mut self) -> Result<(), CDRomError> {
        let status = unsafe {
            cdrom_eject(self.drive_fd).unwrap()
        };

        if status == 2 {
            return Err(CDRomError::DoorLocked)
        }

        Ok(())
    }

    pub fn close(&mut self) -> Result<(), CDRomError> {
        let status = unsafe {
            cdrom_close_tray(self.drive_fd).unwrap()
        };

        match status {
            d if d == Errno::ENOSYS as i32 => Err(CDRomError::Unsupported),
            libc::EBUSY => Err(CDRomError::DoorLocked),
            _ => Ok(()),
        }
    }

    /// Read audio from the CD.
    ///
    /// The buffer will be constructed automatically.
    pub fn read_audio(&mut self, address: Addr, frames: usize) -> Result<Vec<i16>, CDRomError> {
        let mut buf = vec![0i16; (frames * constants::CD_FRAMESIZE_RAW as usize) / 2];

        self.read_audio_into(address, frames, &mut buf)?;

        Ok(buf)
    }

    /// Read audio from the CD into a preallocated buffer.
    ///
    /// The buffer must be large enough to hold the audio for all the frames you want to read.
    /// Since the values are [`i16`]s, the equation for the buffer size is `(n_frames * 2352) / 2`
    pub fn read_audio_into(&mut self, address: Addr, frames: usize, buf: &mut [i16]) -> Result<(), CDRomError> {
        let (addr, addr_format) = match address {
            Addr::Lba(lba) => (AddrUnion { lba }, AddressType::Lba),
            Addr::Msf(msf) => {
                if msf.minute == 0 && msf.second < 2 {
                    panic!("MSF second cannot be less than 2!")
                }

                (AddrUnion { msf }, AddressType::Msf)
            },
        };

        if frames < 1 || frames > 75 {
            panic!("Invalid number of frames!")
        }

        if buf.len() < (frames * constants::CD_FRAMESIZE_RAW as usize) / 2 {
            panic!("Buffer is too small!")
        }

        let mut ra = ReadAudio {
            addr,
            addr_format,
            nframes: frames as i32,
            buf: buf.as_mut_ptr()
        };

        let status = unsafe {
            cdrom_read_audio(self.drive_fd, addr_of_mut!(ra))
        }?;

        if status != 0 {
            return Err(Errno::from_raw(status).into());
        }

        Ok(())
    }

    pub fn read_raw(&mut self, address: Msf) -> Box<[u8; 2352]> {
        // TODO: Make this take LBA values too
        // MSF values are converted to LBA values via this formula:
        // lba = (((m * CD_SECS) + s) * CD_FRAMES + f) - CD_MSF_OFFSET;
        // <https://docs.kernel.org/userspace-api/ioctl/cdrom.html>

        if address.minute == 0 && address.second < 2 {
            panic!("MSF second cannot be less than 2!")
        }

        let mut argument = RawResult {
            cdrom_msf: MsfLong {
                min0: address.minute,
                sec0: address.second,
                frame0: address.frame,
                ..Default::default()
            }
        };

        let result = unsafe {
            cdrom_read_raw(self.drive_fd, addr_of_mut!(argument)).unwrap();

            argument.buffer
        };

        Box::new(result)
    }
}
