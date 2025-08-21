#![no_std]
#![no_main]

use rp2040_hal as hal;

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use hal::{
    clocks::{Clock, init_clocks_and_plls},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

const DELAY: u32 = 1000;

#[entry]
fn main() -> ! {
    let mut p = unsafe { pac::Peripherals::steal() };
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(p.WATCHDOG);
    let sio = Sio::new(p.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        p.XOSC,
        p.CLOCKS,
        p.PLL_SYS,
        p.PLL_USB,
        &mut p.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = hal::gpio::Pins::new(p.IO_BANK0, p.PADS_BANK0, sio.gpio_bank0, &mut p.RESETS);

    let mut led_pin = pins.gpio25.into_push_pull_output();

    loop {
        led_pin.toggle().unwrap();
        delay.delay_ms(DELAY);
    }
}

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
