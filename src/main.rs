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
    let new_line = uefi::CString16::try_from("\n").unwrap();
    let background = Blue;
    system::with_stdout(|stdout| -> uefi::Result {
        stdout.set_color(Black, background)?;

        let mode = stdout.modes().nth(0).unwrap();

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
                stdout.set_cursor_position(0, 0)?;

                let mut prev_color = frame[0][0];

                for row in frame {
                    for color in row {
                        if (prev_color as usize) != (color as usize) {
                            prev_color = color;
                            stdout.set_color(prev_color, background)?;
                        }

                        stdout.output_string(&blockelement_full_block)?;
                    }
                    stdout.output_string(&new_line)?;
                }

                boot::stall(70_000);
            }
        }
    })
    .expect("Error EFI Simple Text Output Protocol");
    boot::stall(1_000_000);
    Status::SUCCESS
}
