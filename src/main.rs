mod constants;
mod structures;
mod platform;

use std::io::Write;
use std::process::exit;

use constants::{AddressType, DiscType, Status};
use structures::{Addr, Msf};


#[macro_use]
extern crate num_derive;

fn main() {
    let mut cd_rom = CDRom::new().unwrap();
    cd_rom.set_lock(true).unwrap();

    println!("Getting drive status...");
    let mut status = cd_rom.status().unwrap();

    if status == Status::NoInfo {
        println!("Cannot get disc status");
        exit(1);
    } else if status == Status::NoDisc {
        println!("No disc inserted!");
        exit(1);
    }

    while status != Status::DiscOK {
        status = cd_rom.status().unwrap();
        if status == Status::TrayOpen {
            cd_rom.close().unwrap();
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("Drive status:\t{:?}", status);

    let disc_type = cd_rom.disc_type();
    println!("Disc type:\t{:?}", disc_type.unwrap_or(DiscType::NoInfo));
    if disc_type != Some(DiscType::Audio) {
        println!("\nNot an audio CD! Will not continue.");
        exit(0);
    }

    let mcn = cd_rom.mcn().unwrap_or_default();
    if mcn != "0000000000000" {
        println!("Disc MCN: {}", mcn);
    }

    let header = cd_rom.toc_header().unwrap();
    println!("Disc contains {} tracks", header.last_track);

    for i in header.first_track..header.last_track {
        let entry = cd_rom.toc_entry(i, AddressType::Msf);

        println!("Track {:>4} -------", entry.track);

        match entry.addr {
            Addr::Lba(a) => {
                let msf = Msf::from_lba(a);
                println!("\tMSF: {:02}:{:02}.{:02}\n\tLBA: {}", msf.minute, msf.second, msf.frame, a);
            }
            Addr::Msf(a) => {
                let lba = a.to_lba();
                println!("\tMSF: {:02}:{:02}.{:02}\n\tLBA: {}", a.minute, a.second, a.frame, lba);
            }
        }
        println!();
    }

    //rip_cd();
}

fn rip_cd() {
    let mut cd_rom = CDRom::new().unwrap();

    println!("Drive status: {:?}", cd_rom.status().unwrap());
    println!("Disc status: {:?}", cd_rom.disc_type().unwrap());

    let mut raw_output = std::fs::File::create("raw_cd").unwrap();
    let mut buffer = vec![32u8; constants::CD_FRAMESIZE_RAW as usize];

    let mut frame = 0i32;
    loop {
        let frame_real = frame % 75;
        let second: i32 = ((frame / 75)) + 2;
        let minute = (second / 60) as u8;

        println!("{:02}:{:02}:{:02} - {}", minute, second % 60, frame_real as u8, frame);

        match cd_rom.read_raw_into(
            Addr::Msf(Msf { minute, second: (second % 60) as u8, frame: frame_real as u8 }),
            &mut buffer,
        ) {
            Ok(s) => s,
            Err(e) => {
                dbg!(e);
                continue;
            },
        };

        let subchannel = cd_rom.subchannel().unwrap();
        //dbg!(subchannel);

        raw_output.write_all(buffer.as_slice()).unwrap();

        frame += 1;
    }
}