#![no_std]
#![no_main]

use panic_halt as _;

use rp2040_pac as pac;

use cortex_m::asm::nop;
use cortex_m_rt::entry;

const XOSC_HZ: u32 = 12_000_000;
const CLK_DELAY_MS: u32 = 64;
const LED_PIN: usize = 25;
const BLINK_DELAY_MS: u32 = 1000;

#[entry]
fn main() -> ! {
    let p = unsafe { pac::Peripherals::steal() };

    p.XOSC
        .ctrl()
        .modify(|_, w| unsafe { w.freq_range().bits(0xaa0) }); // 1_15MHZ
    p.XOSC
        .startup()
        .write(|w| unsafe { w.bits((XOSC_HZ / 1_000_000 + 128) / 256 * CLK_DELAY_MS) });
    p.XOSC.ctrl().modify(|_, w| w.enable().enable());
    while p.XOSC.status().read().stable().bit_is_clear() {}

    p.CLOCKS
        .clk_ref_ctrl()
        .write(|w| unsafe { w.src().bits(0x2) }); // XOSC

    p.RESETS.reset().modify(|_, w| w.io_bank0().set_bit());
    p.RESETS.reset().modify(|_, w| w.io_bank0().clear_bit());
    while p.RESETS.reset_done().read().io_bank0().bit_is_clear() {}

    p.IO_BANK0
        .gpio(LED_PIN)
        .gpio_ctrl()
        .modify(|_, w| w.funcsel().sio());
    p.SIO
        .gpio_oe_set()
        .write(|w| unsafe { w.bits(1 << LED_PIN) });

    loop {
        p.SIO
            .gpio_out_xor()
            .write(|w| unsafe { w.bits(1 << LED_PIN) });

        delay_ms(BLINK_DELAY_MS, &p.TIMER);
    }
}

fn delay_ms(delay: u32, timer: &pac::TIMER) {
    let start = read_time(&timer);
    while read_time(&timer) < start + (delay * 1000) as u64 {
        nop();
    }
}

fn read_time(timer: &pac::TIMER) -> u64 {
    let lt = timer.timelr().read().bits() as u64;
    let ht = timer.timehr().read().bits() as u64;

    (ht << 32) | lt
}

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
