#![no_std]
#![no_main]

use cortex_m::asm::nop;
use panic_halt as _;

use rp_pico as bsp;

use bsp::entry;
use bsp::hal::{pac, sio::Sio};
use embedded_hal::digital::StatefulOutputPin;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let sio = Sio::new(pac.SIO);

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio15.into_push_pull_output();

    loop {
        led_pin.toggle().unwrap();
        for _ in 0..50_000 {
            nop();
        }
    }
}

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
