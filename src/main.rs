mod constants;
mod structures;

use std::os::{fd::IntoRawFd, unix::fs::OpenOptionsExt};
use std::fs::OpenOptions;

use constants::{Operations, IOC_BYTE};
use nix::{ioctl_none, ioctl_none_bad, ioctl_readwrite_bad, libc};

use nix::libc::c_int;
use num_traits::FromPrimitive as _;
use structures::{Addr, Msf0, ReadAudio};

#[macro_use]
extern crate num_derive;

ioctl_none_bad!(cdrom_stop, Operations::to_full(&Operations::Stop));
ioctl_none_bad!(cdrom_start, Operations::to_full(&Operations::Start));
ioctl_none_bad!(cdrom_eject, Operations::to_full(&Operations::Eject));
ioctl_none_bad!(cdrom_close_tray, Operations::to_full(&Operations::CloseTray));
ioctl_none_bad!(cdrom_status, Operations::to_full(&Operations::DriveStatus));
ioctl_readwrite_bad!(cdrom_read_audio, Operations::to_full(&Operations::ReadAudio), structures::ReadAudio);

fn main() {
    const FRAMES: i32 = 75;
    const BYTES_PER_FRAME: i32 = 2352;

    let time = Msf0 {
        minute: 2,
        second: 45,
        frame: 0,
    };

    let address = Addr {
        msf: time,
    };

    let mut buff = [0u8; FRAMES as usize * BYTES_PER_FRAME as usize]; //Frames per second (75) * bytes per frame (2352)

    let mut ra = ReadAudio {
        addr: address,
        addr_format: 0x02,
        nframes: FRAMES,
        buf: buff.as_mut_ptr()
    };

    let cdrom = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_RDONLY)
        .open("/dev/sr0")
        .unwrap();
    let cdrom_fd: c_int = cdrom.into_raw_fd();

    unsafe {
        let result = constants::Status::from_i32(cdrom_status(cdrom_fd).unwrap()).unwrap();
        dbg!(result);
        //dbg!(cdrom_start(cdrom_fd).unwrap_or_default());

        //std::thread::sleep(std::time::Duration::from_secs(1));

        dbg!(cdrom_read_audio(cdrom_fd, std::ptr::addr_of_mut!(ra)).unwrap());
    }

    println!("{:02X?}", &buff[0..10]);
}
