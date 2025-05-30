#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;
use uefi::proto::console::text::Color::{Black, Blue};

mod nyan;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    let blockelement_full_block = uefi::CString16::try_from("█").unwrap();
    let background = Blue;
    system::with_stdout(|stdout| -> uefi::Result {
        stdout.set_color(Black, background)?;

        let mode = stdout.modes().nth(1).unwrap();

        stdout.set_mode(mode)?;

        stdout.clear()?;
        for (i, mode) in stdout.modes().enumerate() {
            info!(
                "{} supported mode {}: {} {}",
                i,
                mode.index(),
                mode.columns(),
                mode.rows()
            );
        }

        loop {
            for frame in nyan::FRAMES {
                let mut prev_color = frame[0][0];

                for (i, row) in frame.iter().enumerate() {
                    stdout.set_cursor_position(0, i)?;
                    for color in row {
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
    boot::stall(1_000_000);
    Status::SUCCESS
}
