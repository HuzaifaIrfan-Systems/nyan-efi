#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Color::{Black, Blue};

use uefi::proto::console::text::Color;

mod frames;

pub const FRAMES: [[[Color; 80]; 25]; 12] = [
    frames::nyan_01::NYAN_01,
    frames::nyan_02::NYAN_02,
    frames::nyan_03::NYAN_03,
    frames::nyan_04::NYAN_04,
    frames::nyan_05::NYAN_05,
    frames::nyan_06::NYAN_06,
    frames::nyan_07::NYAN_07,
    frames::nyan_08::NYAN_08,
    frames::nyan_09::NYAN_09,
    frames::nyan_10::NYAN_10,
    frames::nyan_11::NYAN_11,
    frames::nyan_12::NYAN_12,
];

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let blockelement_full_block = uefi::CString16::try_from("█").unwrap();
    let background = Blue;
    system::with_stdout(|stdout| -> uefi::Result {
        let mode = stdout.modes().nth(0).unwrap();
        stdout.set_mode(mode)?;
        stdout.clear()?;
        stdout.set_color(Black, background)?;

        info!("31 May 2025",);
        info!("Developed by Huzaifa Irfan");

        for (i, mode) in stdout.modes().enumerate() {
            info!(
                "{} supported mode {}: {} {}",
                i,
                mode.index(),
                mode.columns(),
                mode.rows()
            );
        }
        boot::stall(1_000_000);

        loop {
            for frame in FRAMES {
                let mut prev_color = frame[0][0];

                for (i, row) in frame.iter().enumerate() {
                    stdout.set_cursor_position(0, i)?;
                    for color in row[..row.len() - 1].iter() {
                        if (prev_color as usize) != (*color as usize) {
                            prev_color = *color;
                            stdout.set_color(prev_color, background)?;
                        }

                        stdout.output_string(&blockelement_full_block)?;
                    }
                }

                boot::stall(70_000);
            }
        }
    })
    .expect("Error EFI Simple Text Output Protocol");
    boot::stall(100_000_000);
    Status::SUCCESS
}
