#![no_std]
#![no_main]

use rp2040_hal as hal;

use panic_halt as _;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use hal::{pac, sio::Sio};

#[entry]
fn main() -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    let sio = Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
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
