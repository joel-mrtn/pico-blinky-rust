#![no_std]
#![no_main]

use panic_halt as _;

use core::ptr::{read_volatile, write_volatile};
use cortex_m::asm::nop;
use cortex_m_rt::entry;

const XOSC_HZ: u32 = 12_000_000;
const CLK_DELAY_MS: u32 = 64;
const BLINK_DELAY_MS: u32 = 1000;

// === RESETS ===
const RESETS_BASE: usize = 0x4000_c000;
const RESETS_RESET: *mut u32 = (RESETS_BASE + 0x00) as *mut u32;
const RESETS_RESET_DONE: *mut u32 = (RESETS_BASE + 0x08) as *mut u32;

// === CLOCKS ===
const CLOCKS_BASE: usize = 0x4000_8000;
const CLOCKS_CLK_REF_CTRL: *mut u32 = (CLOCKS_BASE + 0x30) as *mut u32;

// === IO_BANK0 ===
const IO_BANK0_BASE: usize = 0x4001_4000;
const IO_BANK0_GPIO25_CTRL: *mut u32 = (IO_BANK0_BASE + 0x0cc) as *mut u32;

// === XOSC ===
const XOSC_BASE: usize = 0x4002_4000;
const XOSC_CTRL: *mut u32 = (XOSC_BASE + 0x00) as *mut u32;
const XOSC_STATUS: *mut u32 = (XOSC_BASE + 0x04) as *mut u32;
const XOSC_STARTUP: *mut u32 = (XOSC_BASE + 0x1c) as *mut u32;

// === TIMER ===
const TIMER_BASE: usize = 0x4005_4000;
const TIMER_TIMEHR: *const u32 = (TIMER_BASE + 0x08) as *const u32;
const TIMER_TIMELR: *const u32 = (TIMER_BASE + 0x0c) as *const u32;

// === SIO ===
const SIO_BASE: usize = 0xd000_0000;
const SIO_GPIO_OUT_XOR: *mut u32 = (SIO_BASE + 0x01c) as *mut u32;
const SIO_GPIO_OE_SET: *mut u32 = (SIO_BASE + 0x024) as *mut u32;

#[entry]
fn main() -> ! {
    unsafe {
        write_volatile(XOSC_CTRL, (0xfab << 12) | 0xaa0);
        write_volatile(
            XOSC_STARTUP,
            (XOSC_HZ / 1_000_000 + 128) / 256 * CLK_DELAY_MS,
        );
        while (read_volatile(XOSC_STATUS) & (1 << 31)) == 0 {}

        write_volatile(CLOCKS_CLK_REF_CTRL, 0x2);

        write_volatile(RESETS_RESET, read_volatile(RESETS_RESET) & !(1 << 5));
        while (read_volatile(RESETS_RESET_DONE) & (1 << 5)) == 0 {}

        write_volatile(IO_BANK0_GPIO25_CTRL, 5);

        write_volatile(SIO_GPIO_OE_SET, 1 << 25);

        loop {
            write_volatile(SIO_GPIO_OUT_XOR, 1 << 25);
            delay_ms(BLINK_DELAY_MS);
        }
    }
}

fn delay_ms(delay: u32) {
    let start = read_time();
    while read_time() < start + (delay * 1000) as u64 {
        nop();
    }
}

fn read_time() -> u64 {
    unsafe {
        let lt = read_volatile(TIMER_TIMELR) as u64;
        let ht = read_volatile(TIMER_TIMEHR) as u64;

        (ht << 32) | lt
    }
}

#[unsafe(link_section = ".boot_loader")]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
